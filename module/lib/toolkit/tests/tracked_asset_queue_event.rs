use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use game_tookit::bevy::prelude::*;

const UPDATE_ITERATION_COUNT: usize = 5000;

#[derive( Debug, Resource, Default )]
struct StoredEvents( Vec< TrackedAssetQueueEvent > );

fn store_asset_events
(
  mut event: EventReader< TrackedAssetQueueEvent >,
  mut storage: ResMut< StoredEvents >,
)
{
  storage.0.extend( event.iter().cloned() );
}

#[test]
fn count_tracked_asset_queue_events()
{
  let mut app = App::new();

  app.add_plugins( MinimalPlugins )
  .add_plugins( AssetPlugin::default() )
  .add_plugins( ImagePlugin::default() )
  .add_plugins( game_tookit::bevy::plugin::al::TrackedAssetLoadingPlugin );

  app.init_resource::< StoredEvents >();

  app.add_systems( Last, store_asset_events );

  {
    while !app.ready() {}

    app.finish();
    app.cleanup();
  }

  {
    app.update();

    let events = &mut app.world.get_resource_mut::< StoredEvents >().unwrap().0;
    let expected = vec![];

    assert_eq!( *events, expected );

    events.clear();
  }

  {
    let mut asset_server =
    {
      let mut asset_server_state : SystemState< TrackedAssetServer > = SystemState::new( &mut app.world );
      asset_server_state.get_mut( &mut app.world )
    };

    let (_im1, _im2, _im3) =
    (
      asset_server.load_tracked::< Image, _ >( "car_black_1.png" ),
      asset_server.load_tracked::< Image, _ >( "car_red_1.png" ),
      asset_server.load_tracked::< Image, _ >( "star.png" ),
    );

    {
      app.update();

      let events = &mut app.world.get_resource_mut::< StoredEvents >().unwrap().0;
      let expected = vec![ TrackedAssetQueueEvent::ResuestsAdded ];

      assert_eq!( *events, expected );
    }

    for _ in 0..UPDATE_ITERATION_COUNT
    {
      app.update();
    }

    {
      let events = &mut app.world.get_resource_mut::< StoredEvents >().unwrap().0;
      let expected = vec!
      [
        TrackedAssetQueueEvent::ResuestsAdded,
        TrackedAssetQueueEvent::RequestProcessed,
      ];

      assert_eq!( *events, expected );
    }
  }
}
