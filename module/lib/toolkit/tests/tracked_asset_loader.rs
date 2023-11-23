use bevy::prelude::*;

use game_tookit::bevy::prelude::*;

#[derive( Resource )]
struct Timeout( std::time::Duration );

#[derive( Clone, Copy, Event )]
enum WaitTimeEnded
{
  Timeout,
  EmptyQueue,
}

fn wait_for_load_with_timeout
(
  time : Res< '_, Time >,
  timeout : Res< '_, Timeout >,
  mut ev_queue_empty : EventReader< '_, '_, AssetLoadQueueEmptyEvent >,
  mut ev_end_waiting : EventWriter< '_, WaitTimeEnded >,
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

#[derive( Debug, Default, Resource )]
struct ShouldExit( bool );

fn count_events
(
  mut results : ResMut< '_, Results >,
  expected_results : ResMut< '_, ExpectedResults >,

  mut ev_start_load : EventReader< '_, '_, StartAssetLoadEvent >,
  mut ev_finish_load : EventReader< '_, '_, FinishAssetLoadEvent >,
  mut ev_fail_load : EventReader< '_, '_, FailedAssetLoadEvent >,
  mut ev_queue_empty : EventReader< '_, '_, AssetLoadQueueEmptyEvent >,
  mut ev_new_reguests : EventReader< '_, '_, AssetLoadRequestsAddedEvent >,

  ev_end_waiting : EventReader< '_, '_, WaitTimeEnded >,

  mut exit : ResMut< '_, ShouldExit >,
)
{
  results.start += ev_start_load.iter().count();
  results.finish += ev_finish_load.iter().count();
  results.fail += ev_fail_load.iter().count();
  results.queue_empty += ev_queue_empty.iter().count();
  results.new_requests += ev_new_reguests.iter().count();

  // println!("{:?}", results);

  if !ev_end_waiting.is_empty()
  {
    assert!(results.start == expected_results.start);
    assert!(results.finish == expected_results.finish);
    assert!(results.fail == expected_results.fail);
    assert!(results.new_requests == expected_results.new_requests);

    exit.0 = true;
  }
}

fn startup( mut asset_server : TrackedAssetServer <'_> )
{
  asset_server.load_tracked::< Image, _ >( "car_red_1.png" );
  asset_server.load_tracked::< Image, _ >( "car_black_1.png" );
  asset_server.load_tracked::< Image, _ >( "star.png" );
}

#[test]
fn count_produced_events()
{
  let mut app = App::new();

  app.add_plugins( MinimalPlugins )
  .add_plugins( AssetPlugin::default() )
  .add_plugins( ImagePlugin::default() )
  .add_plugins( game_tookit::bevy::plugin::al::TrackedAssetLoadingPlugin );

  app.add_event::< WaitTimeEnded >();

  app.init_resource::< Time >()
  .init_resource::< Results >()
  .init_resource::< ShouldExit >()
  .insert_resource( Timeout( std::time::Duration::from_secs_f32( 10.0 ) ) )
  .insert_resource
  (
    ExpectedResults
    {
      start : 3,
      finish : 3,
      fail : 0,
      new_requests : 1,
    }
  );

  app.add_systems( Startup, startup );
  app.add_systems
  (
    Update,
    (
      wait_for_load_with_timeout,
      count_events.after(wait_for_load_with_timeout),
    ),
  );

  app.set_runner
  (
    |mut app|
    {
      while !app.ready() {}

      app.finish();
      app.cleanup();

      loop
      {
        app.update();

        if app.world.resource::< ShouldExit >().0
        {
          break;
        }
      }
    }
  );

  app.run();
}
