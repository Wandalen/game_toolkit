#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

//!
//! Aggregate and augment math for game development.
//!

/// Alias to float type.
pub type S = f32;
/// Accuracy factor.
pub const Eps : d2::S = 1e-8;

pub use bevy_math::*;
pub mod d2;
pub mod d3;

// xxx : move
pub mod value_of;

// pub mod vec2;
// pub use vec2::*;
// pub mod cbox;
// pub use cbox::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use bevy_math::prelude::*;
  pub use super::d2::prelude::*;
  pub use super::d3::prelude::*;
}
