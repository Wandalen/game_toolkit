//!
//! Define interface for 2D box centered.
//!

use crate::*;
use d2::VExtTrait;
use std::borrow::Cow;

mod collider;
pub use collider::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  // use crate::*;
  // pub use bevy_math::prelude::*;
  pub use super::CboxTrait;
  pub use super::CboxMutTrait;
  pub use super::collider::prelude::*;
}

///
/// Requires implementation of centered box.
///
/// In one way or another has information about center and size of a box in 2D.
/// Does not implement mutable methods, for mutable methods look `CboxMutTrait`.
///
pub trait CboxTrait
{

  /// Get center of the centered box.
  fn center( &self ) -> d2::V;
  /// Get size of the centered box.
  fn size( &self ) -> d2::V;
  /// Cast to centered box. Return refence if can. Otherwise produce a new instance of type Cbox.
  fn as_centered_box( &'_ self ) -> Cow< '_, Cbox >;
  /// Convert to Cbox producing a new instace of the type.
  fn to_centered_box( &self ) -> Cbox;
  /// Convert to 2D matrix.
  #[ inline ]
  fn to_intervals( &self ) -> [ [ S ; 2 ] ; 2 ]
  {
    let min = self.min();
    let max = self.max();
    [ [ min.x, max.x ], [ min.y, max.y ] ]
  }
  /// Construct new instace based on a center and size of box.
  fn FromCenterAndSize< IntoCenter, IntoSize >( center : IntoCenter, size : IntoSize ) -> Self
  where
    IntoCenter : Into< d2::V >,
    IntoSize : Into< d2::V >,
  ;
  /// Construct new instace based on minimum and maximum.
  #[ inline ]
  fn FromMinMax< IntoMin, IntoMax >( min : IntoMin, max : IntoMax ) -> Self
  where
    Self : Sized,
    IntoMin : Into< d2::V >,
    IntoMax : Into< d2::V >,
  {
    let min = min.into();
    let max = max.into();
    let center = ( min + max ) * 0.5;
    let size = max - min;
    Self::FromCenterAndSize( center, size )
  }
  /// Construct new instace based on minimum and maximum.
  #[ inline ]
  fn FromIntervals< IntoX, IntoY >( x : IntoX, y : IntoY ) -> Self
  where
    Self : Sized,
    IntoX : Into< d2::V >,
    IntoY : Into< d2::V >,
  {
    let x = x.into();
    let y = y.into();
    let center = d2::V::new( ( x.x + x.y ) * 0.5, ( y.x + y.y ) * 0.5 );
    let size = d2::V::new( x.y - x.x, y.y - y.x );
    Self::FromCenterAndSize( center, size )
  }

  /// Half size.
  #[ inline ]
  fn hsize( &self ) -> d2::V
  {
    self.size() / 2.0
  }
  /// Minimal point. Assuming left and bottom are lowest sides it is the most left bottom corner of box.
  #[ inline ]
  fn min( &self ) -> d2::V
  {
    self.center() - self.hsize()
  }
  /// Maximum point. Assuming left and bottom are lowest sides it is the most right top corner of box.
  #[ inline ]
  fn max( &self ) -> d2::V
  {
    self.center() + self.hsize()
  }

  // xxx : add extra argument for seed.
  /// Random point in the region.
  #[ inline ]
  fn random_point( &self ) -> d2::V
  {
    let r = self.center() + ( d2::V::Random() - 0.5 ) * self.size();
    // println!( "in cbox {:?} random point is {:?}", &self, &r );
    r
  }

}

///
/// Mutable extension of centered box `CboxTrait`.
///
pub trait CboxMutTrait : CboxTrait
{

  /// Get center mutably.
  fn center_set( &mut self, src : &d2::V );
  /// Get size mutably.
  fn size_set( &mut self, src: &d2::V );

  /// Move center to fit current cbox into another cbox.
  ///
  /// Return true if any change where done.
  fn cbox_fit
  (
    &mut self,
    cbox2 : &impl CboxTrait,
  ) -> bool
  {
    let hsize1 = self.size() / 2.0;
    let center0 = self.center();
    let center2 = center0.clamp( cbox2.min() + hsize1, cbox2.max() - hsize1 );
    if center0 == center2
    {
      return false;
    }
    self.center_set( &center2 );
    true
  }

}

///
/// Implementation of centered box.
///
/// Has information about center and size of a box in 2D.
///

#[ derive( Debug, Copy, Clone ) ]
pub struct Cbox
{
  /// Center.
  pub center : d2::V,
  /// Size or extand. Half-size along each axis from center is how big box is.
  pub size : d2::V,
}

impl CboxTrait for Cbox
{
  #[ inline ]
  fn center( &self ) -> d2::V
  {
    self.center
  }
  #[ inline ]
  fn size( &self ) -> d2::V
  {
    self.size
  }
  #[ inline ]
  fn as_centered_box( &'_ self ) -> Cow< '_, Cbox >
  {
    Cow::Borrowed( self )
  }
  #[ inline ]
  fn to_centered_box( &self ) -> Cbox
  {
    self.clone()
  }
  #[ inline ]
  // fn FromCenterAndSize( center : d2::V, size : d2::V ) -> Self
  fn FromCenterAndSize< IntoCenter, IntoSize >( center : IntoCenter, size : IntoSize ) -> Self
  where
    IntoCenter : Into< d2::V >,
    IntoSize : Into< d2::V >,
  {
    let center = center.into();
    let size = size.into();
    Self { center, size }
  }
}

impl CboxMutTrait for Cbox
{

  #[ inline ]
  fn center_set( &mut self, src : &d2::V )
  {
    self.center = *src;
  }

  #[ inline ]
  fn size_set( &mut self, src : &d2::V )
  {
    self.size = *src;
  }

}

//

#[cfg(test)]
mod tests
{
  use super::*;
  use diagnostics_tools::*;

  #[ test ]
  fn to_intervals_test()
  {

    //

    let cbox = Cbox::FromCenterAndSize( d2::V::new( 0.0, 1.0 ), d2::V::new( 2.0, 3.0 ) );
    let got = cbox.to_intervals();
    let exp = [ [ -1.0, 1.0 ], [ -0.5, 2.5 ] ];
    a_id!( got ,exp );

    //

  }

}
