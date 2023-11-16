//!
//! Pawn of the game.
//!

// use crate::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Enumerate of pawns.
///
#[ derive( Debug ) ]
pub enum Pawn
{
  /// Non-player character.
  Npc,
  /// Player.
  Player,
}

impl Default for Pawn
{
  fn default() -> Self
  {
    Self::Npc
  }
}
