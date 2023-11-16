#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

//! Toolkit for terminal-based games.

/// Namespace to include with asterisk.
pub mod prelude
{
}

use game_tookit as kit;
pub use crossterm;
pub use termcolor;
pub use image;

pub mod draw;

use kit::generic::dyn_error::DynError;
use draw::ImageDrawOptions;
use image::DynamicImage;
use crossterm::event::{poll, read, Event};
use std::time::Duration;

//

use image::io::Reader as ImageReader;

/// Run the demo.
pub fn run() -> Result< (), DynError >
{

  use game_tookit::generic::path_of_workspace::path_of_workspace_cd;
  path_of_workspace_cd();

  let mut o = ImageDrawOptions::default();
  o.screen_size = Some( ( 80, 80 ) ).into();
  o.screen_offset = ( 10, 5 ).into();
  o.using_256_colors = true.into();
  o.is_transparent = false.into();
  // o.resize_filter = FilterType::Lanczos3.into();
  // o.absolute = true;
  dbg!( &o );
  let img : DynamicImage = ImageReader::open( "asset/img/car_red_1.png" )?.decode()?;
  // let img : DynamicImage = ImageReader::open( "asset/img/photo/underwater.jpg" )?.decode()?;
  let r = draw::draw( &img, &o );
  dbg!( r.unwrap() );

  o.using_256_colors = false.into();
  // o.resize_filter = FilterType::Nearest;
  dbg!( &o );
  let r = draw::draw( &img, &o );
  dbg!( r.unwrap() );

  loop
  {
    if poll( Duration::from_millis( 25 ) )?
    {
      match read()?
      {
        Event::FocusGained => println!( "FocusGained" ),
        Event::FocusLost => println!( "FocusLost" ),
        Event::Key( event ) => println!( "{:?}", event ),
        Event::Mouse( event ) => println!( "{:?}", event ),
        Event::Paste( data ) => println!( "Pasted {:?}", data ),
        Event::Resize( width, height ) => println!( "New size {}x{}", width, height ),
      }
    }
    else
    {
      // Timeout expired and no `Event` is available
    }
  }
  // Ok( () )
}
