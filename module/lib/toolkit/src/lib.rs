//!
//! Helpers for game developments.
//!

#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

#[ cfg( feature = "generic" ) ]
pub mod generic;
#[ cfg( feature = "bevy" ) ]
pub mod bevy;
pub use sealed::sealed;
#[ cfg( feature = "rand" ) ]
pub use rand;

// pub use generic::*;
// pub use bevy::*;

// #[ cfg( feature = "generic_dyn_result" ) ]
// pub use generic::runnable::{ Runnable, DynResult };

/// Namespace to include with asterisk.
pub mod prelude
{
  #[ cfg( feature = "generic" ) ]
  pub use super::generic::prelude::*;
  #[ cfg( feature = "bevy" ) ]
  pub use super::bevy::prelude::*;
}
pub use prelude::*;