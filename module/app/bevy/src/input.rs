//!
//! Input management.
//!

use crate::*;

pub use physics::{ MovableComponent, PlayerComponent };
pub use plugin::dpad_control::{ ActionState, DpadAction };

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
}

/// Handle 2D move input.
pub fn player_input_fn
(
  mut quiery : Query< '_, '_, &mut MovableComponent, With< PlayerComponent > >,
  action_state_query : Query< '_, '_, &ActionState< DpadAction > >,
)
{
  if let Ok( action_state ) = action_state_query.get_single()
  {
    if let Ok( mut movable ) = quiery.get_single_mut()
    {
      if action_state.pressed( DpadAction::Move )
      {
        let axis_pair = action_state.axis_pair( DpadAction::Move ).unwrap();
        let direction = Vec2::new( axis_pair.x(), axis_pair.y() );
        movable.0.movable_player_input( Some( direction ) );
      }
      else
      {
        movable.0.movable_player_input( None );
      }
    }
  }
}
