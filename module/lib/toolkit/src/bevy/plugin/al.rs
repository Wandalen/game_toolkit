//!
//! To track assets loading process.
//!
use bevy::
{
    asset::{ Asset, AssetEvents, AssetPath },
    prelude::*,
};

/// Namespace to include with asterisk.
pub mod prelude
{
    pub use super::TrackedAssetServer;
    pub use super::{ AssetLoadEvent, TrackedAssetQueueEvent };
}

///
/// To track assets loading process.
///
#[derive( Debug, Default )]
pub struct TrackedAssetLoadingPlugin;

/// Alias for the plugin defined here.
pub type Plugin = TrackedAssetLoadingPlugin;

impl bevy::app::Plugin for TrackedAssetLoadingPlugin
{
  fn build( &self, app : &mut App )
  {
    app.init_resource::<TrackedAssetsQueue>();

    app.add_event::<AssetLoadEvent>()
    .add_event::<TrackedAssetQueueEvent>();

    app.add_systems
    (
      AssetEvents,
      (
        TrackedAssetsQueue::add_assets_to_queue,
        TrackedAssetsQueue::process_assets_status
        .after( TrackedAssetsQueue::add_assets_to_queue ),
      ),
    );
  }
}

///
/// Event to signal asset load status.
///
#[derive( Debug, Event, Clone, PartialEq, Eq )]
pub enum AssetLoadEvent
{
  /// Tracked asset load started.
  Started(HandleUntyped),
  /// Tracked asset load finished succesfully.
  Finished(HandleUntyped),
  /// Tracked asset load failed.
  Failed(HandleUntyped),
}

///
/// Event to signal global tracked assets queue status.
///
#[derive( Debug, Event, Clone, Copy, PartialEq, Eq )]
pub enum TrackedAssetQueueEvent
{
  /// New tracked requests for asset loading were added.
  ResuestsAdded,
  /// All tracked requests for asset loading were processed.
  RequestProcessed,
}

///
/// Extends standard asset server adding extra logic to track progress.
///
#[allow( missing_debug_implementations )]
#[derive( bevy::ecs::system::SystemParam )]
pub struct TrackedAssetServer< 'w >
{
  asset_server: Res< 'w, AssetServer >,
  ev_asset_track: EventWriter< 'w, AssetLoadEvent >,
}

impl< 'w > TrackedAssetServer< 'w >
{
  /// Get an underlying AssetServer
  pub fn asset_server( &self ) -> &AssetServer
  {
    &self.asset_server
  }

  /// Load asset while tracking the loading process
  pub fn load_tracked< 'a, T : Asset, P : Into< AssetPath< 'a > > >( &mut self, path : P ) -> Handle<T>
  {
    let asset = self.asset_server.load(path);
    self.ev_asset_track
    .send(AssetLoadEvent::Started(asset.clone_untyped()));

    asset
  }
}

///
/// Storage of tracked assets waiting to be loaded
///
#[derive( Debug, Default, Resource )]
pub struct TrackedAssetsQueue
{
  handles : bevy::utils::HashSet< HandleUntyped >,
}

impl TrackedAssetsQueue
{
  /// System to handle loading process of assets
  pub fn add_assets_to_queue
  (
    mut ev_asset : EventReader< '_, '_, AssetLoadEvent >,
    mut ev_queue : EventWriter< '_, TrackedAssetQueueEvent >,
    mut queue : ResMut< '_, TrackedAssetsQueue >,
  )
  {
    let queue_is_empty = queue.handles.is_empty();
    let mut new_added = false;

    for event in ev_asset.iter()
    {
      if let AssetLoadEvent::Started( handle ) = event
      {
        new_added = true;
        queue.handles.insert( handle.clone() );
      }
    }

    if new_added && queue_is_empty
    {
      ev_queue.send( TrackedAssetQueueEvent::ResuestsAdded );
    }
  }

  /// System to handle loading process of images and replace it with default if error will happen.
  pub fn process_assets_status
  (
    asset_server : Res< '_, AssetServer >,
    mut queue : ResMut< '_, TrackedAssetsQueue >,

    mut ev_asset : EventWriter< '_, AssetLoadEvent >,
    mut ev_queue : EventWriter< '_, TrackedAssetQueueEvent >,
  )
  {
    use bevy::asset::LoadState;

    let initial_queue_len = queue.handles.len();

    queue
    .handles
    .retain
    (
      |handle|
        match asset_server.get_load_state( handle )
        {
          LoadState::NotLoaded | LoadState::Loading => true,
          LoadState::Loaded | LoadState::Unloaded =>
          {
            ev_asset.send( AssetLoadEvent::Finished( handle.clone() ) );
            false
          }
          LoadState::Failed =>
          {
            ev_asset.send( AssetLoadEvent::Failed( handle.clone() ) );
            false
          }
        }
    );

    if queue.handles.is_empty() && initial_queue_len != 0
    {
      ev_queue.send( TrackedAssetQueueEvent::RequestProcessed );
    }
  }
}
