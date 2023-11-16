//!
//! Interface to run anything.
//!

use crate::*;
pub use generic::dyn_error::DynResult;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use super::{ Runnable, DynResult };
}

///
/// Interface to run anything.
///
pub trait Runnable
{
  /// Create instance with default arguments if necessary and run it.
  fn Run() -> DynResult< () >;
}

