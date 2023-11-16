//!
//! Generic helpers.
//!

#[ cfg( feature = "generic_map_of_vectors" ) ]
pub mod map_of_vectors;
#[ cfg( feature = "generic_path_of_workspace" ) ]
pub mod path_of_workspace;
#[ cfg( feature = "generic_dyn_error" ) ]
pub mod dyn_error;
#[ cfg( feature = "generic_runnable" ) ]
pub mod runnable;

/// Namespace to include with asterisk.
pub mod prelude
{
  #[ cfg( feature = "generic_map_of_vectors" ) ]
  pub use super::map_of_vectors::prelude::*;
  #[ cfg( feature = "generic_path_of_workspace" ) ]
  pub use super::path_of_workspace::prelude::*;
  #[ cfg( feature = "generic_dyn_error" ) ]
  pub use super::dyn_error::prelude::*;
  #[ cfg( feature = "generic_runnable" ) ]
  pub use super::runnable::prelude::*;
}
