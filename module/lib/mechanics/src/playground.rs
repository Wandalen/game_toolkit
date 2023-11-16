//!
//! Physical playground of the game.
//!

// use game_math::*;

use crate::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Physical word represented as box.
///

#[ derive( Debug ) ]
pub struct Playground
{
  /// Centered box representing physical word.
  pub cbox : d2::Cbox,
  /// Inverted mass. Should be zero for playground.
  pub inv_mass : S,
}

impl Default for Playground
{
  fn default() -> Self
  {
    let center = d2::V::ZERO;
    // let size = d2::V::ONE;
    // let size = d2::V::new( 64.0, 32.0 );
    let size = Game::PlaygroundSize;
    // let cbox = d2::Cbox::FromCenterAndSize( d2::V::ZERO, size );
    let cbox = d2::Cbox { center, size };
    let inv_mass = 0.;
    Self { inv_mass, cbox }
  }
}

