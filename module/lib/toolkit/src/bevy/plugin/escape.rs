
//!
//! Handle escape keyboard button press on keyboard to terminate application.
//!

use bevy::prelude::*;
use bevy::app::AppExit;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Handle escape keyboard button press on keyboard to terminate application.
///
#[ derive( Debug ) ]
pub struct EscapePlugin;
/// Alias for the plugin defined here.
pub type Plugin = EscapePlugin;
impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    app
    .add_systems( Update, escape_handler_fn )
    ;
  }
}

/// Handle escape keyboard button press on keyboard to terminate application.
pub fn escape_handler_fn
(
  keyboard_input : Res< '_, Input< KeyCode > >,
  mut events: EventWriter< '_, AppExit >,
)
{
  if keyboard_input.just_pressed( KeyCode::Escape )
  {
    println!( "Exiting..." );
    events.send( AppExit );
  }
}
