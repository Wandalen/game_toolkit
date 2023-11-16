//!
//! Use bevy game engine to impelemnt visual and other aspectso of mechanics of the game.
//!
#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

//! Bevy-based version of game.

pub use game_mechanics as mechanics;
pub use game_tookit as kit;
pub use kit::bevy;
pub use kit::bevy::plugin;
pub use kit::bevy::plugin::al;
pub use game_math as math;
pub use math::{ d2, d3 };
pub use bevy::prelude::*;

pub mod assets;
pub mod input;
pub mod mechanics_bevy;
pub mod sound;
pub mod physics;
pub mod visual;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use super::math::prelude::*;
  pub use super::assets::prelude::*;
  pub use super::input::prelude::*;
  pub use super::mechanics_bevy::prelude::*;
  pub use super::sound::prelude::*;
  pub use super::physics::prelude::*;
  pub use super::visual::prelude::*;
  pub use super::kit::prelude::*;
}

///
/// Main plugin for the game.
///
/// Main entity of the crate.
///
#[ derive( Debug, Default ) ]
pub struct GamePlugin;
/// Alias for the main entity of the crate.
pub type Game = GamePlugin;

impl GamePlugin
{
  /// Size of one cell.
  pub const CellSize : f32 = 20.0;
}

impl bevy::app::Plugin for GamePlugin
{
  fn build( &self, app : &mut App )
  {
    let workspace_path = kit::generic::path_of_workspace::path_of_workspace();

    app

    .add_plugins( plugin::default::Plugin { workspace_path } )
    .add_plugins( plugin::escape::Plugin {} )
    .add_plugins( plugin::dpad_control::Plugin {} )
    .add_plugins( plugin::assets_watch::Plugin {} )
    // .add_plugins( bevy::diagnostic::FrameTimeDiagnosticsPlugin {} )
    // .add_systems( Update, bevy::diagnostic::FrameTimeDiagnosticsPlugin::diagnostic_system )
    // .add_plugins( bevy::diagnostic::EntityCountDiagnosticsPlugin )
    .add_plugins( bevy::diagnostic::SystemInformationDiagnosticsPlugin )
    // .add_plugins( plugin::fps::ScreenDiagnosticsPlugin { timestep : 250.0, ..default() } )
    // .add_plugins( plugin::fps::ScreenFrameDiagnosticsPlugin )
    .add_plugins( plugin::al::Plugin::default() )

    .configure_sets( FixedUpdate,
    (
      physics::Phase::Pre,
      physics::Phase::PositionIntegration,
      physics::Phase::PositionResolve,
      physics::Phase::VelocityIntegration,
      physics::Phase::VelocityResolve,
      physics::Phase::Post
    ).chain() )
    .init_resource::< physics::CollisionPairsResource >()
    .insert_resource( FixedTime::new_from_secs( 1.0 / mechanics::Game::PhysicsFps ) )
    .add_systems( PreStartup, physics::setup_world_fn.in_set( visual::Phase::Pre ) )
    .add_systems( Startup, physics::setup_movable_player_fn.in_set( visual::Phase::In ) )
    .add_systems( Startup, physics::setup_movable_npcs_fn.in_set( visual::Phase::In ) )

    .add_systems( FixedUpdate, physics::movable_integrate1_fn.in_set( physics::Phase::PositionIntegration ) )
    .add_systems( FixedUpdate, physics::movable_playground_collide_solve_fn.in_set( physics::Phase::PositionResolve ) )
    .add_systems( FixedUpdate, physics::movable_movable_collision_solve_fn.in_set( physics::Phase::PositionResolve ) )
    .add_systems( FixedUpdate, physics::movable_integrate2_fn.in_set( physics::Phase::VelocityIntegration ) )
    .add_systems( FixedUpdate, physics::movable_velocity_resolve_fn.in_set( physics::Phase::VelocityResolve ) )
    .add_systems( FixedUpdate, input::player_input_fn.in_set( physics::Phase::Post )  )

    .configure_sets( Startup, ( visual::Phase::Pre, visual::Phase::In, visual::Phase::Post ).chain() )
    .insert_resource( ClearColor( Color::BLACK ) )
    .insert_resource( Msaa::Sample4 )
    .add_systems( Startup, visual::setup_world_fn )
    .add_systems( Update, visual::movable_transformation_update_fn )
    .add_systems( Update, visual::camera_transformation_update_fn )
    .add_systems( Update, visual::delayed_setup_world_fn )

    .add_event::< sound::SoundEvent >()
    .add_systems( Startup, sound::setup_fn )
    .add_systems( PostUpdate, sound::play_fn )

    ;
  }
}

//

impl kit::Runnable for GamePlugin
{
  fn Run() -> kit::DynResult< () >
  {
    App::new()
    .add_plugins( GamePlugin::default() )
    .run()
    ;
    Ok( () )
  }

}
