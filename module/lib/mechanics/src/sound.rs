//!
//! Sound of the game.
//!

// use crate::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Enumerate sounds.
//

#[ derive( Debug, Clone, Copy, Hash, Eq, PartialEq ) ]
pub enum Sound
{
  /// No sound.
  None,
  /// Sound of hit into border.
  HitBorder,
  /// Sound of hit of two pawns.
  HitPawns,
}

impl Default for Sound
{
  fn default() -> Self
  {
    Sound::None
  }
}
