//!
//! Physics.
//!

use crate::*;

// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;

// use math::prelude::*;
// use math::Cbox;
use assets::AssetImageEvent;
use mechanics::Playground;
use sound::SoundEvent;
pub use mechanics::movable::*;

// use mechanics_bevy::prelude::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
  // pub use plugin::prelude::*;
}

/// Player marker.
#[ derive( Component, Default, Debug ) ]
pub struct PlayerComponent;

/// NPC marker.
#[ derive( Component, Default, Debug ) ]
pub struct NpcComponent;

/// Object capable to be moved.
#[ derive( Component, Default, Debug ) ]
pub struct MovableComponent
(
  pub Movable,
);

/// Marker to focus the object.
#[ derive( Component, Default, Debug ) ]
pub struct CameraFocusComponent;

/// Game world.
#[ derive( Resource, Default, Debug ) ]
pub struct PlaygroundResource( pub Playground );

/// List of paris of entities which collided.
#[ derive( Resource, Default, Debug ) ]
pub struct CollisionPairsResource( pub Vec< ( Entity, Entity ) > );

/// Phases of physics simulation
#[ derive( SystemSet, Debug, Default, Hash, PartialEq, Eq, Clone ) ]
pub enum Phase
{
  /// First one.
  Pre,
  /// Integration of position.
  PositionIntegration,
  #[ default ]
  /// Resolution of position.
  PositionResolve,
  /// Phase of integration of velocities.
  VelocityIntegration,
  /// Resolution of velocity.
  VelocityResolve,
  /// Last one.
  Post,
}

/// Setup physics for the world.
pub fn setup_world_fn
(
  mut commands : Commands< '_, '_ >,
  // window : Query< '_, '_, &Window, With< PrimaryWindow > >,
)
{

  // playground

  // let window = window.get_single().unwrap();
  // let size = Vec2::new( window.width(), window.height() );
  // let size = Vec2::new( 64.0, 32.0 );
  // let cbox = d2::Cbox::FromCenterAndSize( Vec2::ZERO, size );
  let playground = Playground::default();
  commands.insert_resource
  (
    PlaygroundResource( playground ),
  );

}

/// Setup player.
pub fn setup_movable_player_fn
(
  mut commands : Commands< '_, '_ >,
  mut asset_image_events : EventWriter< '_, AssetImageEvent >,
  asset_server : Res< '_, AssetServer >,
)
{
  let movable = MovableComponent( Movable::PawnPlayer() );
  movable.visual_pawn_player( &mut commands, &mut asset_image_events, asset_server );
}

/// Setup NPCs.
pub fn setup_movable_npcs_fn
(
  mut commands : Commands< '_, '_ >,
  mut asset_image_events : EventWriter< '_, AssetImageEvent >,
  asset_server : Res< '_, AssetServer >,
  playground : Res< '_, PlaygroundResource >,
)
{
  for _ in 0..mechanics::Game::nNpc
  {
    let movable = MovableComponent( Movable::PawnNpc( playground.0.cbox ) );
    movable.visual_pawn_npc( &mut commands, &mut asset_image_events, Res::clone( &asset_server ) );
  }
}

/// Integrate position of movables.
pub fn movable_integrate1_fn
(
  mut query : Query< '_, '_, ( &mut MovableComponent, ) >,
  time : Res< '_, Time >,
)
{
  let delta_seconds = time.delta_seconds();
  for( mut movable, ) in query.iter_mut()
  {
    if movable.0.integrate1( delta_seconds )
    {
    }
  }
}

/// Resolve collisions with playground borders and play sound.
pub fn movable_movable_collision_solve_fn
(
  mut movables : Query< '_, '_, ( Entity, &mut MovableComponent, ), >,
  mut sound_event : EventWriter< '_, SoundEvent >,
  mut collisions: ResMut< '_, CollisionPairsResource >,
  // playground : Res< '_, PlaygroundResource >,
)
{
  collisions.0.clear();

  let mut combinations = movables.iter_combinations_mut();
  while let Some( pair ) = combinations.fetch_next()
  // for e in combinations.iter()
  {
    let [ ( aentity, mut a ), ( bentity, mut b ) ] = pair;
    match Movable::movable_movable_collide( &mut a.0, &mut b.0 )
    {
      Some( ( _collision, sound ) ) =>
      {
        collisions.0.push( ( aentity, bentity ) );
        sound_event.send( SoundEvent{ sound } )
      },
      // Some( ( collision, sound ) ) => {},
      None => {},
    }
  }

}

/// Resolve collisions with playground borders and play sound.
pub fn movable_playground_collide_solve_fn
(
  mut movables : Query< '_, '_, ( &mut MovableComponent, ), >,
  mut sound_event : EventWriter< '_, SoundEvent >,
  playground : Res< '_, PlaygroundResource >,
)
{
  for( mut movable, ) in movables.iter_mut()
  {
    // movable.0.playground_collide( &playground.0 );
    match movable.0.playground_collide( &playground.0 )
    {
      Some( ( _collision, sound ) ) => sound_event.send( SoundEvent{ sound } ),
      // Some( ( collision, sound ) ) => {},
      None => {},
    }
  }
}

/// Integrate position and adjust velocity after constraints.
pub fn movable_integrate2_fn
(
  mut query : Query< '_, '_, ( &mut MovableComponent, ) >,
  time : Res< '_, Time >,
)
{
  let delta_seconds = time.delta_seconds();
  for( mut movable, ) in query.iter_mut()
  {
    if movable.0.integrate2( delta_seconds )
    {
    }
  }
}

/// Integrate position and adjust velocity after constraints.
pub fn movable_velocity_resolve_fn
(
  #[ allow( unused_mut ) ]
  mut movables : Query< '_, '_, ( &mut MovableComponent, ) >,
  collisions : Res< '_, CollisionPairsResource >,
  // time : Res< '_, Time >,
)
{
  if collisions.0.len() == 0
  {
    return;
  }
  println!( "collisions : {}", collisions.0.len() );
  for ( e1, e2 ) in collisions.0.iter().cloned()
  {
    let ( mut m1, mut m2 ) = unsafe
    {
      debug_assert!( e1 != e2 );
      ( movables.get_unchecked( e1 ).unwrap(), movables.get_unchecked( e2 ).unwrap() )
    };
    collision_velocity_resolve( &mut m1.0.0, &mut m2.0.0 );
  }

}
