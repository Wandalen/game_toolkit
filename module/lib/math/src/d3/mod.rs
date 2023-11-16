
//!
//! 3D math for games.
//!

pub mod vec;
pub use vec::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use bevy_math::prelude::*;
  pub use super::vec::prelude::*;
}
