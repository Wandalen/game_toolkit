//!
//! Draw an raster to terminal.
//!

/// Namespace to include with asterisk.
pub mod prelude
{
}

use crate::*;
use kit::generic::dyn_error::{ DynError, err_make };
use ansi_colours::ansi256_from_rgb;
use image::{ ImageBuffer, DynamicImage, Rgba, imageops::FilterType };
use termcolor::{ BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor };
use crossterm::{ execute, cursor::{ MoveRight, MoveTo, MoveToPreviousLine } };
use core::ops::{ Deref, DerefMut };

/// Color of is_transparent region if it's set to be drawn.
const TransparentColor1 : ( u8, u8, u8 ) = ( 192, 192, 192 );
/// Color of is_transparent region if it's set to be drawn.
const TransparentColor2 : ( u8, u8, u8 ) = ( 128, 128, 128 );
// Because algorithm draw 2 pixels along Y axis per character. Charactqer with upper half.
const UppoerHalfChar : &str = "\u{2580}";
// Because algorithm draw 2 pixels along Y axis per character. Charactqer with lower half.
const LowerHalfChar : &str = "\u{2584}";

// Some( console_color( ( pixel.2[ 0 ], pixel.2[ 1 ], pixel.2[ 2 ] ) ) )

///
/// Which color to consider transparent if any.
///
#[ derive( Debug, Clone, Copy ) ]
pub enum TransparencyCondition
{
  /// No pixels are considered transparent.
  None,
  /// Consider pixel having alpha zero as completely transparent.
  AlphaZero,
  /// Consider pixel having color equal to referenced as completely transparent.
  Color( Rgba< u8 > ),
}

//

impl TransparencyCondition
{

  fn is_transparent( &self ) -> Box< dyn Fn( &Rgba< u8 > ) -> bool + '_ >
  {
    match self
    {
      Self::None => Box::new( | _ | false ),
      Self::AlphaZero => Box::new( | color | color[ 3 ] == 0 ),
      Self::Color( tcolor ) => Box::new( move | color | color == tcolor ),
    }
  }
}

impl Default for TransparencyCondition
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    TransparencyCondition::AlphaZero
  }
}

/// To skip drawing is_transparent pixels. If false then is_transparent pixels drawn as checkboard.
#[ derive( Debug, Clone, Copy ) ]
pub struct IsTransparent( bool );
// xxx : use type constructor
impl Default for IsTransparent
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( true )
  }
}
impl From< bool > for IsTransparent
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}
impl From< IsTransparent > for bool
{
  #[ inline( always ) ]
  fn from( src : IsTransparent ) -> Self
  {
    src.0
  }
}
impl Deref for IsTransparent
{
  type Target = bool;
  #[ inline( always ) ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl DerefMut for IsTransparent
{
  #[ inline( always ) ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

/// Use the extended 256-color palette.
#[ derive( Debug, Clone, Copy ) ]
pub struct Using256Colors( bool );
impl Using256Colors
{

  /// Convert color to color which console understand.
  #[ inline ]
  fn console_color( self ) -> Box< dyn Fn( ( u8, u8, u8 ) ) -> Color >
  {
    let r = match self.0
    {
      true => | color : ( u8, u8, u8 ) |
      {
        Color::Rgb( color.0, color.1, color.2 )
      },
      false => | color : ( u8, u8, u8 ) |
      {
        Color::Ansi256( ansi256_from_rgb( color ) )
      }
    };
    Box::new( r )
  }

  // /// Make function to get color for pixel which is transparent. To draw chess board pattern.
  // #[ inline ]
  // fn transparency_color( self ) -> Box< dyn Fn( ( u32, u32 ) ) -> Color >
  // {
  //   let r = match self.0
  //   {
  //     true => | id : ( u32, u32 ) |
  //     {
  //       let rgb = if id.0 % 2 == id.1 % 2
  //       {
  //         TransparentColor2
  //       }
  //       else
  //       {
  //         TransparentColor1
  //       };
  //       Color::Rgb( rgb.0, rgb.1, rgb.2 )
  //     },
  //     false => | id : ( u32, u32 ) |
  //     {
  //       let rgb = if id.0 % 2 == id.1 % 2
  //       {
  //         TransparentColor2
  //       }
  //       else
  //       {
  //         TransparentColor1
  //       };
  //       Color::Ansi256( ansi256_from_rgb( rgb ) )
  //     }
  //   };
  //   Box::new( r )
  // }

}
impl Default for Using256Colors
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( true )
  }
}
impl From< bool > for Using256Colors
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}
impl From< Using256Colors > for bool
{
  #[ inline( always ) ]
  fn from( src : Using256Colors ) -> Self
  {
    src.0
  }
}
impl Deref for Using256Colors
{
  type Target = bool;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl DerefMut for Using256Colors
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

/// Absolute position. Use current position of cursor if set to false.
#[ derive( Debug, Clone, Copy ) ]
pub struct IsAbsolute( bool );
impl Default for IsAbsolute
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( false )
  }
}
impl From< bool > for IsAbsolute
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}
impl From< IsAbsolute > for bool
{
  #[ inline( always ) ]
  fn from( src : IsAbsolute ) -> Self
  {
    src.0
  }
}
impl Deref for IsAbsolute
{
  type Target = bool;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl DerefMut for IsAbsolute
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

/// Screen position where to draw.
#[ derive( Debug, Clone, Copy ) ]
pub struct ScreenOffset( i16, i16 );
impl Default for ScreenOffset
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( 0, 0 )
  }
}
impl From< ( i16, i16 ) > for ScreenOffset
{
  #[ inline( always ) ]
  fn from( src : ( i16, i16 ) ) -> Self
  {
    Self( src.0, src.1 )
  }
}
impl From< ScreenOffset > for ( i16, i16 )
{
  #[ inline( always ) ]
  fn from( src : ScreenOffset ) -> Self
  {
    ( src.0, src.1 )
  }
}
impl Deref for ScreenOffset
{
  type Target = ( i16, i16 );
  fn deref( &self ) -> &Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}
impl DerefMut for ScreenOffset
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}

/// Resize filter to use to balance between perofrmance and quality.
#[ derive( Debug, Clone, Copy ) ]
pub struct ResizeFilter( FilterType );
impl Default for ResizeFilter
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( FilterType::Nearest )
  }
}
impl From< FilterType > for ResizeFilter
{
  #[ inline( always ) ]
  fn from( src : FilterType ) -> Self
  {
    Self( src )
  }
}
impl From< ResizeFilter > for FilterType
{
  #[ inline( always ) ]
  fn from( src : ResizeFilter ) -> Self
  {
    src.0
  }
}
impl Deref for ResizeFilter
{
  type Target = FilterType;
  fn deref( &self ) -> &Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}
impl DerefMut for ResizeFilter
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}

/// Size of the area on the screen to be included in the rendered image.
#[ derive( Debug, Clone, Copy ) ]
pub struct ScreenSize( Option< ( u32, u32 ) > );
impl Default for ScreenSize
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( None )
  }
}
impl From< ( u32, u32 ) > for ScreenSize
{
  #[ inline( always ) ]
  fn from( src : ( u32, u32 ) ) -> Self
  {
    Self( Some( ( src.0, src.1 ) ) )
  }
}
impl From< Option< ( u32, u32 ) > > for ScreenSize
{
  #[ inline( always ) ]
  fn from( src : Option< ( u32, u32 ) > ) -> Self
  {
    Self( src )
  }
}
impl From< ScreenSize > for Option< ( u32, u32 ) >
{
  #[ inline( always ) ]
  fn from( src : ScreenSize ) -> Self
  {
    src.0
  }
}
impl Deref for ScreenSize
{
  type Target = Option< ( u32, u32 ) >;
  fn deref( &self ) -> &Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}
impl DerefMut for ScreenSize
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    unsafe { core::mem::transmute( self ) }
  }
}

///
/// Options to draw image to a console.
///
#[ derive( Debug, Default ) ]
pub struct ImageDrawOptions
{
  /// To skip drawing is_transparent pixels. If false then is_transparent pixels drawn as checkboard.
  pub is_transparent : IsTransparent,
  /// Which color to consider transparent if any.
  pub transparency_condition : TransparencyCondition,
  /// Use the extended 256-color palette.
  pub using_256_colors : Using256Colors,
  /// Absolute position. Use current position of cursor if set to false.
  pub is_absolute : IsAbsolute,
  /// Screen position where to draw.
  pub screen_offset : ScreenOffset,
  /// Resize filter to use to balance between perofrmance and quality.
  pub resize_filter : ResizeFilter,
  /// Size of the area on the screen to be included in the rendered image.
  pub screen_size : ScreenSize,
}

///
/// Options to draw image to a console.
///
#[ derive( Debug, Default ) ]
pub struct ImageBufferDrawOptions
{
  /// To skip drawing is_transparent pixels. If false then is_transparent pixels drawn as checkboard.
  pub is_transparent : IsTransparent,
  /// Which color to consider transparent if any.
  pub transparency_condition : TransparencyCondition,
  /// Use the extended 256-color palette.
  pub using_256_colors : Using256Colors,
  /// Absolute position. Use current position of cursor if set to false.
  pub is_absolute : IsAbsolute,
  /// Screen position where to draw.
  pub screen_offset : ScreenOffset,
}
impl From< &ImageDrawOptions > for ImageBufferDrawOptions
{
  #[ inline( always ) ]
  fn from( src : &ImageDrawOptions ) -> Self
  {
    Self
    {
      is_transparent : src.is_transparent,
      transparency_condition : src.transparency_condition,
      using_256_colors : src.using_256_colors,
      is_absolute : src.is_absolute,
      screen_offset : src.screen_offset,
    }
  }
}

///
/// Draw image to a console.
///
/// Returns size of drawed region in chars if success.
///

pub fn draw
(
  img : &DynamicImage,
  o : &ImageDrawOptions,
)
-> Result< ( u32, u32 ), DynError >
{
  let mut stream = BufferedStandardStream::stdout( ColorChoice::Always );
  img_draw_to_stream( &mut stream, &img, &o )
}

///
/// Draw image to a stream.
///
/// Returns size of drawed region in chars if success.
///

pub fn img_draw_to_stream
(
  stdout : &mut impl WriteColor,
  img : &DynamicImage,
  o : &ImageDrawOptions,
)
-> Result< ( u32, u32 ), DynError >
{
  use std::borrow::Cow;

  // transform image

  let img : Cow< '_, _ > = if o.screen_size.is_some()
  {
    let sz = o.screen_size.unwrap();
    let img: DynamicImage = img.resize( sz.0, sz.1 * 2, *o.resize_filter );
    Cow::Owned( img )
  }
  else
  {
    Cow::Borrowed( img )
  };
  // let sz = img.dimensions();
  // let mut row : Vec< ColorSpec > = vec![ ColorSpec::new() ; sz.0 as usize ];
  let img_buffer = img.to_rgba8();

  img_buffer_draw_to_stream( stdout, &img_buffer, &o.into() )
}

///
/// Draw image to a stream.
///
/// Returns size of drawed region in chars if success.
///

pub fn img_buffer_draw_to_stream
(
  stdout : &mut impl WriteColor,
  img_buffer : &ImageBuffer< Rgba< u8 >, Vec< u8 > >,
  o : &ImageBufferDrawOptions,
)
-> Result< ( u32, u32 ), DynError >
{

  let sz = img_buffer.dimensions();
  let mut row : Vec< ColorSpec > = vec![ ColorSpec::new() ; sz.0 as usize ];

  // offset first

  if *o.is_absolute
  {
    if o.screen_offset.0 >= 0 && o.screen_offset.1 >= 0
    {
      execute!( stdout, MoveTo( o.screen_offset.0 as u16, ( ( o.screen_offset.1 + 1 ) / 2 ) as u16 ) )?;
    }
    else
    {
      return err_make( &format!( "Negative value {:?}!", o.screen_offset ) )
    }
  }
  else
  {
    if o.screen_offset.1 < 0
    {
      // MoveUp if negative
      execute!( stdout, MoveToPreviousLine( ( ( -o.screen_offset.1 + 1 ) / 2 ) as u16 ) )?;
    } else
    {
      // Unline MoveDown writeln! scrolls.
      let c = ( o.screen_offset.1 + 1 ) / 2;
      for _ in 0..c
      {
        writeln!( stdout )?;
      }
    }
  }

  // draw row by row, drawing 2 pixels along y axis along single row of characters.

  let console_color = o.using_256_colors.console_color();
  let is_transparent = o.transparency_condition.is_transparent();
  for( curr_row, img_row ) in img_buffer.enumerate_rows()
  {
    let is_even_row = curr_row % 2 == 0;
    let is_last_row = curr_row == sz.1 - 1;

    // move right if x offset is specified
    if o.screen_offset.0 > 0 && ( !is_even_row || is_last_row )
    {
      execute!( stdout, MoveRight( o.screen_offset.0 as u16 ) )?;
    }


    for pixel in img_row
    {
      let color = if ( is_transparent )( pixel.2 )
      {
        if *o.is_transparent
        {
          None
        }
        else
        {
          let color = [ TransparentColor1, TransparentColor2 ][ ( curr_row % 2 == pixel.0 % 2 ) as usize ];
          Some( console_color( color ) )
        }
      }
      else
      {
        Some( console_color( ( pixel.2[ 0 ], pixel.2[ 1 ], pixel.2[ 2 ] ) ) )
      };

      // To draw chess board pattern drawing 2 pixels along y axis per character.
      let color_spec : &mut ColorSpec = &mut row[ pixel.0 as usize ];
      stdout_write_color( stdout, color_spec, color, is_even_row, is_last_row )?;
    }

    // reset and next line

    if !is_even_row && !is_last_row
    {
      stdout.reset()?;
      writeln!( stdout, "\r" )?;
    }

  }

  // reset, next line and flush

  stdout.reset()?;
  writeln!( stdout )?;
  stdout.flush()?;

  // return effective size in characters
  Ok( ( sz.0, sz.1 / 2 + sz.1 % 2 ) )
}

/// Draw one character to stream.

fn stdout_write_color
(
  stdout : &mut impl WriteColor,
  color_spec : &mut ColorSpec,
  color0 : Option< Color >,
  is_even_row : bool,
  is_last_row : bool,
)
-> Result< (), DynError >
{
  let color;
  let chr;
  let mut color2;

  if is_even_row
  {
    color_spec.set_bg( color0 );
    if !is_last_row
    {
      return Ok( () )
    }
  }
  else
  {
    color_spec.set_fg( color0 );
  }

  // Because algorithm draw 2 pixels along Y axis per character bottom pixel should is_transparent if image has odd pixels along Y axis
  if is_last_row
  {
    color2 = ColorSpec::new();
    if let Some( bg ) = color_spec.bg()
    {
      color2.set_fg( Some( *bg ) );
      chr = UppoerHalfChar;
    }
    else
    {
      execute!( stdout, MoveRight( 1 ) )?;
      return Ok( () );
    }
    color = &color2;
  }
  else
  {
    // Because algorithm draw 2 pixels along Y axis per character there are 4 possible combinations.
    match( color_spec.fg(), color_spec.bg() )
    {
      ( None, None ) =>
      {
        // completely transparent
        execute!( stdout, MoveRight( 1 ) )?;
        return Ok( () );
      }
      ( Some( bottom ), None ) =>
      {
        // only top transparent
        color2 = ColorSpec::new();
        color2.set_fg( Some( *bottom ) );
        color = &color2;
        chr = LowerHalfChar;
      }
      ( None, Some( top ) ) =>
      {
        // only bottom transparent
        color2 = ColorSpec::new();
        color2.set_fg( Some( *top ) );
        color = &color2;
        chr = UppoerHalfChar;
      }
      ( Some( _ ), Some( _ ) ) =>
      {
        color = color_spec;
        chr = LowerHalfChar;
      }
    }
  }

  stdout.set_color( color )?;
  write!( stdout, "{}", chr )?;

  Ok( () )
}
