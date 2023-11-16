//!
//! Default entry point of the game.
//!
#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

pub use app_bevy as app;
use app::prelude::*;
use app::Game;

// pub use app_fyrox as app;
// use app::prelude::*;
// use app::Game;

// fn main()
// {
//
//   App::new()
//   .add_plugins( Game::default() )
//   .run()
//   ;
//
//   // let mut executor = app::Executor::new();
//   // executor.add_plugin_constructor( app::GameConstructor );
//   // executor.run()
//
// }

fn main()
{
  Game::Run().unwrap()
}
