//!
//! All image assets of the game.
//!

use crate::*;
use pawn::Pawn;

/// Namespace to include with asterisk.
pub mod prelude
{
}

/// Enumerate asset images of the game.
#[ derive( Debug ) ]
pub enum AssetImage
{
  /// No image.
  No,
  /// Pawn.
  Pawn( Pawn ),
  /// Pattern 1.
  Background,
}

impl Default for AssetImage
{
  fn default() -> Self
  {
    Self::No
  }
}
