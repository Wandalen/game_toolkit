//!
//! Handle ASDW and arrows emitting events.
//!

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub use leafwing_input_manager::action_state::ActionState;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use leafwing_input_manager::prelude::*;
}

///
/// Handle ASDW and arrows emitting events.
///
#[ derive( Debug ) ]
pub struct DpadControlPlugin;
/// Alias for the plugin defined here.
pub type Plugin = DpadControlPlugin;
impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    app
    .add_plugins( InputManagerPlugin::< DpadAction >::default() )
    .add_systems( Startup, dpad_control )
    ;
  }
}

/// Defined actions.
#[ derive( Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect ) ]
pub enum DpadAction
{
  /// Just move into either one of 4 direction.
  Move,
}

fn dpad_control
(
  mut commands : Commands< '_, '_ >,
)
{
  commands.spawn
  (
    (
      InputManagerBundle::< DpadAction >
      {
        input_map: InputMap::new
        ([
          (
            VirtualDPad
            {
              up : KeyCode::W.into(),
              down : KeyCode::S.into(),
              left : KeyCode::A.into(),
              right : KeyCode::D.into(),
            },
            DpadAction::Move,
          ),
          (
            VirtualDPad
            {
              up : KeyCode::Up.into(),
              down : KeyCode::Down.into(),
              left : KeyCode::Left.into(),
              right : KeyCode::Right.into(),
            },
            DpadAction::Move,
          ),
          (
            VirtualDPad
            {
              up : KeyCode::Numpad8.into(),
              down : KeyCode::Numpad2.into(),
              left : KeyCode::Numpad4.into(),
              right : KeyCode::Numpad6.into(),
            },
            DpadAction::Move,
          ),
        ])
        .build(),
        ..default()
      },
    )
  );

}
