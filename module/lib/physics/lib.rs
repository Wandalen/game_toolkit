//!
//! Physical game engine.
//!
#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

pub use game_math as math;
use game_math::*;

pub mod collider;

pub use collider::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use game_math::prelude::*;
  pub use super::collider::prelude::*;
}

// ///
// /// Definition of Game.
// ///
//
// #[ derive( Debug ) ]
// pub struct Game;
//
// impl Game
// {
//   /// Size of playground in cells.
//   pub const PlaygroundSize : d2::V = d2::V::new( 4.1, 32.0 );
//   /// Size of a pawn.
//   pub const PawnRadius : d2::S = 1.;
//   /// Player speed.
//   pub const MovableNpcSpeed : d2::S = 2.;
//   /// Player speed.
//   pub const MovablePlayerAcceleration : d2::S = 200.;
//   /// Frames per seconds for physics simulation.
//   pub const PhysicsFps : d2::S = 60.0;
//   /// Friction of the world. Velocity is multiplied by `1.0 - Game::Friction * self.friction` on each integration.
//   pub const Friction : d2::S = 1e-3;
//   /// Default friction of an pawn. Velocity is multiplied by `1.0 - Game::Friction * self.friction` on each integration.
//   pub const DefaultPawnFriction : d2::S = 1.;
//   /// Default friction of player.
//   pub const DefaultPlayerFriction : d2::S = 50.;
//   /// Number of NPCs for simulation.
//   pub const nNpc : usize = 16;
//
// }
