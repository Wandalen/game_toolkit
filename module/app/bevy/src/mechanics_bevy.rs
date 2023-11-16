//!
//! Bevy-specific aspectso f mechanics.
//!

// use crate::*;
// use math::Cbox;
// use math::Vec3Swizzles;

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
}

/// Local wrapper.
#[ derive( Debug ) ]
pub struct W< T >( T );

//

// impl From< ( &Transform, &Image ) > for W< Cbox >
// {
//   fn from( src : ( &Transform, &Image ) ) -> Self
//   {
//     let center = src.0.translation.xy();
//     let size = src.1.size();
//     W( Cbox { center, size } )
//   }
// }
//
// //
//
// impl From< &Window > for W< Cbox >
// {
//   fn from( src : &Window ) -> Self
//   {
//     let size = Vec2::new( src.width(), src.height() );
//     let center = size / 2.0;
//     W( Cbox { center, size } )
//   }
// }
