use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use game_tookit::bevy::prelude::*;

const UPDATE_ITERATION_COUNT: usize = 5000;

#[derive( Debug, Resource, Default )]
struct StoredEvents( Vec< AssetLoadEvent > );

fn store_asset_events( mut event : EventReader< AssetLoadEvent >, mut storage : ResMut< StoredEvents > )
{
  storage.0.extend( event.iter().cloned() );
}

#[test]
fn count_asset_load_events()
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
    events.clear();
  }

  {
    let mut asset_server =
    {
      let mut asset_server_state : SystemState< TrackedAssetServer > = SystemState::new( &mut app.world );
      asset_server_state.get_mut( &mut app.world )
    };

    let (im1, im2, im3) =
    (
      asset_server.load_tracked::< Image, _ >( "car_black_1.png" ),
      asset_server.load_tracked::< Image, _ >( "car_red_1.png" ),
      asset_server.load_tracked::< Image, _ >( "star.png" ),
    );

    {
      app.update();

      let events = &mut app.world.get_resource_mut::< StoredEvents >().unwrap().0;
      let expected = vec!
      [
        AssetLoadEvent::Started( im1.clone_untyped() ),
        AssetLoadEvent::Started( im2.clone_untyped() ),
        AssetLoadEvent::Started( im3.clone_untyped() ),
      ];

      assert_eq!( *events, expected );

      events.clear();
    }

    for _ in 0..UPDATE_ITERATION_COUNT
    {
      app.update();
    }

    {
      let events = &mut app.world.get_resource_mut::< StoredEvents >().unwrap().0;
      let expected = vec!
      [
        AssetLoadEvent::Finished( im1.clone_untyped() ),
        AssetLoadEvent::Finished( im2.clone_untyped() ),
        AssetLoadEvent::Finished( im3.clone_untyped() ),
      ];

      assert_eq!( events.len(), 3 );
      assert!( events.contains( &expected[0] ) );
      assert!( events.contains( &expected[1] ) );
      assert!( events.contains( &expected[2] ) );
    }
  }
}
