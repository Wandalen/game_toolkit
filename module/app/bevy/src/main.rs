//!
//! Use bevy game engine to impelemnt visual and other aspectso of mechanics of the game.
//!
#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

use app_bevy as app;
use app::prelude::*;
use app::Game;

fn main()
{
  Game::Run().unwrap()
}
