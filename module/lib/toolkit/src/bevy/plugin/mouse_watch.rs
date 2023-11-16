//!
//! Track all mouse events.
//!

use bevy::prelude::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

use bevy::input::mouse::{ MouseButtonInput, MouseWheel };


///
/// Track all mouse events.
///
#[ derive( Debug ) ]
pub struct MouseWatchPlugin;
/// Alias for the plugin defined here.
pub type Plugin = MouseWatchPlugin;
impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    app
    .add_systems( Update, mouse_input_fn )
    ;
  }
}

fn mouse_input_fn
(
  mut button_input : EventReader< '_, '_, MouseButtonInput >,
  mut move_input : EventReader< '_, '_, CursorMoved >,
  mut wheel_input : EventReader< '_, '_, MouseWheel >,
)
{
  for event in button_input.iter()
  {
    println!( "Mouse button : {event:?}" );
  }
  for event in move_input.iter()
  {
    println!( "Mouse move : {event:?}" );
  }
  for event in wheel_input.iter()
  {
    println!( "Mouse wheel : {event:?}" );
  }
}
