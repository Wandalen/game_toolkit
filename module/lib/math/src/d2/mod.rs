
//!
//! 2D math for games.
//!

pub mod cbox;
pub use cbox::*;
pub mod collision;
pub use collision::*;
pub mod sphere;
pub use sphere::*;
pub mod vec;
pub use vec::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use bevy_math::prelude::*;
  pub use super::cbox::prelude::*;
  pub use super::collision::prelude::*;
  pub use super::sphere::prelude::*;
  pub use super::vec::prelude::*;
}
