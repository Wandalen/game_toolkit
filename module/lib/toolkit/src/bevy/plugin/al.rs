//!
//! To track assets loading process
//!

use bevy::
    prelude::*
;
use bevy::
    asset::{Asset, AssetPath};

///
/// To track assets loading process.
///
#[derive( Debug, Default ) ]
pub struct AssetLoadingPlugin;

/// Alias for the plugin defined here.
pub type Plugin = AssetLoadingPlugin;

impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    app.init_resource::<AssetLoadingQueue>();

    app.add_event::<StartAssetLoadEvent>()
      .add_event::<FinishAssetLoadEvent>()
      .add_event::<FailedAssetLoadEvent>()
      .add_event::<AssetLoadRequestsAddedEvent>()
      .add_event::<AssetLoadQueueEmptyEvent>();

    app.add_systems
      (
        Update,
          (
            AssetLoadingQueue::add_assets_to_queue,
            AssetLoadingQueue::process_assets_status,
          ),
      );
  }
}

///
/// Event to signal that loading is started.
///

#[derive( Event, Debug ) ]
pub struct StartAssetLoadEvent( HandleUntyped );

///
/// Event to signal that loading is finished.
///

#[derive( Event, Debug ) ]
pub struct FinishAssetLoadEvent( HandleUntyped );

///
/// Event to signal that loading failed.
///

#[derive( Event, Debug )]
pub struct FailedAssetLoadEvent( HandleUntyped );

///
/// Event to signal that all requested assets are loaded.
///

#[derive( Event, Debug )]
pub struct AssetLoadQueueEmptyEvent;

///
/// Event to signal that new asset load requests were added.
///

#[derive( Event, Debug )]
pub struct AssetLoadRequestsAddedEvent;

///
/// Extends standard asset server adding extra logic to track progress.
///
#[derive(SystemParam)]
pub struct TrackedAssetServer<'w>
{
    asset_server : Res< 'w, AssetServer >,
    ev_asset_track : EventWriter< 'w, StartAssetLoadEvent >,
}

impl<'w> TrackedAssetServer<'w> {
  pub fn asset_server( &self ) -> &AssetServer
  {
    &self.asset_server
  }

  pub fn load_tracked< 'a, T : Asset, P : Into< AssetPath<'a> > >( &mut self, path : P ) -> Handle< T >
  {
    let asset = self.asset_server.load( path );
    self.ev_asset_track
      .send( StartAssetLoadEvent( asset.clone_untyped() ) );
    asset
  }
}

///
/// Queue of tracked assets waiting to be loaded
///
#[derive( Debug, Default, Resource ) ]
pub struct AssetLoadingQueue
{
  handles : Vec< HandleUntyped >,
}

impl AssetLoadingQueue
{
  /// System to handle loading process of assets
  pub fn add_assets_to_queue
  (
    mut ev_load_requests : EventReader< StartAssetLoadEvent >,
    mut ev_added : EventWriter< AssetLoadRequestsAddedEvent >,
    mut queue : ResMut< AssetLoadingQueue >,
  )
  {
    if !ev_load_requests.is_empty()
    {
      ev_added.send( AssetLoadRequestsAddedEvent );
    }

    for event in ev_load_requests.iter()
    {
      queue.handles.push( event.0.clone() );
    }
  }

  /// System to handle loading process of images and replace it with default if error will happen.
  pub fn process_assets_status
  (
    asset_server : Res< AssetServer >,
    mut queue : ResMut< AssetLoadingQueue >,
    mut ev_finish : EventWriter< FinishAssetLoadEvent >,
    mut ev_failed : EventWriter< FailedAssetLoadEvent >,
    mut ev_empty : EventWriter< AssetLoadQueueEmptyEvent >,
  )
  {
    use bevy::asset::LoadState;

    let initial_queue_len = queue.handles.len();

    queue
      .handles
      .retain(|handle|
        match asset_server.get_load_state( handle )
        {
          LoadState::NotLoaded | LoadState::Loading => true,
          LoadState::Loaded | LoadState::Unloaded =>
          {
            ev_finish.send( FinishAssetLoadEvent( handle.clone() ) );
            false
          }
          LoadState::Failed =>
          {
            ev_failed.send( FailedAssetLoadEvent( handle.clone() ) );
            false
          }
        });

    if queue.handles.is_empty() && initial_queue_len != 0
    {
      ev_empty.send( AssetLoadQueueEmptyEvent );
    }
  }
}

#[cfg( test )]
mod tests {
  use bevy::prelude::*;
  use bevy::app::AppExit;
  use bevy::log::LogPlugin;

  #[derive( Resource )]
  struct Timeout(std::time::Duration);

  #[derive( Copy, Event )]
  enum WaitTimeEnded
  {
    Timeout,
    EmptyQueue,
  }

  fn wait_for_load_with_timeout
  (
    time : Res< Time >,
    timeout : Res< Timeout >,
    mut ev_queue_empty : EventReader< super::AssetLoadQueueEmptyEvent >,
    mut ev_end_waiting : EventWriter< WaitTimeEnded >,
  )
  {
    if time.startup().elapsed() > timeout.0
    {
      ev_end_waiting.send( WaitTimeEnded::Timeout );
    }

    if let Some(_) = ev_queue_empty.iter().next()
    {
      ev_end_waiting.send( WaitTimeEnded::EmptyQueue );
    }
  }

  #[derive( Debug, Default, Resource )]
  struct Results
  {
    start : usize,
    finish : usize,
    fail : usize,
    queue_empty : usize,
    new_requests : usize,
  }

  #[derive( Resource )]
  struct ExpectedResults
  {
    start : usize,
    finish : usize,
    fail : usize,
    new_requests : usize,
  }

  fn count_events
  (
    mut results : ResMut< Results >,
    expected_results : ResMut< ExpectedResults >,

    mut ev_start_load : EventReader< super::StartAssetLoadEvent >,
    mut ev_finish_load : EventReader< super::FinishAssetLoadEvent >,
    mut ev_fail_load : EventReader< super::FailedAssetLoadEvent >,
    mut ev_queue_empty : EventReader< super::AssetLoadQueueEmptyEvent >,
    mut ev_new_reguests : EventReader< super::AssetLoadRequestsAddedEvent >,

    ev_end_waiting : EventReader< WaitTimeEnded >,

    mut ev_exit : EventWriter< AppExit >,
  )
  {
    results.start += ev_start_load.iter().count();
    results.finish += ev_finish_load.iter().count();
    results.fail += ev_fail_load.iter().count();
    results.queue_empty += ev_queue_empty.iter().count();
    results.new_requests += ev_new_reguests.iter().count();

    // println!("{results:?}");

    if !ev_end_waiting.is_empty()
    {
      assert!(results.start == expected_results.start);
      assert!(results.finish == expected_results.finish);
      assert!(results.fail == expected_results.fail);
      assert!(results.new_requests == expected_results.new_requests);

      ev_exit.send( AppExit );
    }
  }

  fn startup( mut asset_server : super::TrackedAssetServer )
  {
    asset_server.load_tracked::< Image, _ >( "fire_01.png" );
    asset_server.load_tracked::< Image, _ >( "fire_02.png" );
    asset_server.load_tracked::< Image, _ >( "fire_sprite_atlas.png" );
  }

  #[test]
  fn count_produced_events()
  {
    let mut app = App::new();

    app.add_plugins( MinimalPlugins )
       .add_plugins( LogPlugin::default() )
       .add_plugins( AssetPlugin::default() )
       .add_plugins( ImagePlugin::default() )
       .add_plugins( super::TrackedAssetLoadingPlugin );

    app.add_event::< WaitTimeEnded >();

    app.init_resource::< Time >()
       .init_resource::< Results >()
       .insert_resource( Timeout( std::time::Duration::from_secs_f32( 10.0 ) ) )
       .insert_resource(
          ExpectedResults
          {
            start : 3,
            finish : 3,
            fail : 0,
            new_requests : 1,
          }
        );

    app.add_systems( Startup, startup );
    app.add_systems(
      Update,
      (
        wait_for_load_with_timeout,
        count_events.after(wait_for_load_with_timeout),
      ),
    );

    app.run();

    loop {}
    // loop {
    //     app.update();
    // }
  }
}
