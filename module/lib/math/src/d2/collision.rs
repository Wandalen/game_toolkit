//!
//! Define interface for 2D box centered.
//!

use crate::*;
// use d2::VExtTrait;
// use std::borrow::Cow;

/// Namespace to include with asterisk.
pub mod prelude
{
  // use crate::*;
  // pub use bevy_math::prelude::*;
  // pub use super::SphereTrait;
  // pub use super::SphereMutTrait;
  // pub use super::CollideSphereTrait;
}

///
/// Describe collision in 2D: point, normal and depth.
///
#[ derive( Debug, Default, PartialEq ) ]
pub struct Collision
{
  /// Point of collision.
  pub point : d2::V,
  /// Normal of collision.
  pub normal : d2::V,
  /// Depth of collision.
  pub depth : S,
}
