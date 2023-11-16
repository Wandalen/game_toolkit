//!
//! Collection of useful plugins for Bevy.
//!

#[ cfg( feature = "bevy_plugin_al" ) ]
pub mod al;
#[ cfg( feature = "bevy_plugin_assets_watch" ) ]
pub mod assets_watch;
#[ cfg( feature = "bevy_plugin_default" ) ]
pub mod default;
#[ cfg( feature = "bevy_plugin_dpad_control" ) ]
pub mod dpad_control;
#[ cfg( feature = "bevy_plugin_escape" ) ]
pub mod escape;
#[ cfg( feature = "bevy_plugin_fps" ) ]
pub mod fps;
#[ cfg( feature = "bevy_plugin_mouse_watch" ) ]
pub mod mouse_watch;

/// Namespace to include with asterisk.
pub mod prelude
{
  // use bevy::prelude::*;
  #[ cfg( feature = "bevy_plugin_al" ) ]
  pub use super::al::prelude::*;
  #[ cfg( feature = "bevy_plugin_assets_watch" ) ]
  pub use super::assets_watch::prelude::*;
  #[ cfg( feature = "bevy_plugin_default" ) ]
  pub use super::default::prelude::*;
  #[ cfg( feature = "bevy_plugin_dpad_control" ) ]
  pub use super::dpad_control::prelude::*;
  #[ cfg( feature = "bevy_plugin_escape" ) ]
  pub use super::escape::prelude::*;
  #[ cfg( feature = "bevy_plugin_fps" ) ]
  pub use super::fps::prelude::*;
  #[ cfg( feature = "bevy_plugin_mouse_watch" ) ]
  pub use super::mouse_watch::prelude::*;
}
