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
  pub use super::SphereTrait;
  pub use super::SphereMutTrait;
  pub use super::CollideSphereTrait;
  pub use super::collider::prelude::*;
}

///
/// Requires implementation of centered box.
///
/// In one way or another has information about center and radius of a box in 2D.
/// Does not implement mutable methods, for mutable methods look `SphereMutTrait`.
///
pub trait SphereTrait
{

  /// Get center of the centered box.
  fn center( &self ) -> d2::V;
  /// Get radius of the centered box.
  fn radius( &self ) -> S;
  /// Cast to centered box. Return refence if can. Otherwise produce a new instance of type Sphere.
  fn as_sphere( &'_ self ) -> Cow< '_, Sphere >;
  /// Convert to Sphere producing a new instace of the type.
  fn to_sphere( &self ) -> Sphere;

  /// Construct new instace based on a center and radius of box.
  fn FromCenterAndRadius< IntoCenter, IntoRadius >( center : IntoCenter, radius : IntoRadius ) -> Self
  where
    IntoCenter : Into< d2::V >,
    IntoRadius : Into< S >,
  ;

  /// Half radius.
  #[ inline ]
  fn half_radius( &self ) -> S
  {
    self.radius() / 2.0
  }
  /// Minimal point. Assuming left and bottom are lowest sides it is the most left bottom corner of box.
  #[ inline ]
  fn min( &self ) -> d2::V
  {
    self.center() - self.half_radius()
  }
  /// Maximum point. Assuming left and bottom are lowest sides it is the most right top corner of box.
  #[ inline ]
  fn max( &self ) -> d2::V
  {
    self.center() + self.half_radius()
  }

  // xxx : add extra argument for seed.
  /// Random point in the region.
  #[ inline ]
  fn random_point( &self ) -> d2::V
  {
    let r = self.center() + ( d2::V::Random() ).sqrt() * self.radius();
    // println!( "in shpere {:?} random point is {:?}", &self, &r );
    r
  }

}

///
/// Mutable extension of centered box `SphereTrait`.
///
pub trait SphereMutTrait : SphereTrait
{

  /// Get center mutably.
  fn center_set( &mut self, src : &d2::V );
  /// Get radius mutably.
  fn radius_set( &mut self, src: S );

  /// Move center to fit current shpere into another shpere.
  ///
  /// Return true if any change where done.
  fn shpere_fit
  (
    &mut self,
    shpere2 : &impl SphereTrait,
  ) -> bool
  {
    let radius1 = self.radius() / 2.0;
    let center0 = self.center();
    let center2 = center0.clamp( shpere2.min() + radius1, shpere2.max() - radius1 );
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
/// Has information about center and radius of a box in 2D.
///

#[ derive( Debug, Copy, Clone, PartialEq ) ]
pub struct Sphere
{
  /// Center.
  pub center : d2::V,
  /// Size or extand. Half-radius along each axis from center is how big box is.
  pub radius : S,
}

impl SphereTrait for Sphere
{
  #[ inline ]
  fn center( &self ) -> d2::V
  {
    self.center
  }
  #[ inline ]
  fn radius( &self ) -> S
  {
    self.radius
  }
  #[ inline ]
  fn as_sphere( &'_ self ) -> Cow< '_, Sphere >
  {
    Cow::Borrowed( self )
  }
  #[ inline ]
  fn to_sphere( &self ) -> Sphere
  {
    self.clone()
  }
  #[ inline ]
  // fn FromCenterAndRadius( center : d2::V, radius : S ) -> Self
  fn FromCenterAndRadius< IntoCenter, IntoRadius >( center : IntoCenter, radius : IntoRadius ) -> Self
  where
    IntoCenter : Into< d2::V >,
    IntoRadius : Into< S >,
  {
    let center = center.into();
    let radius = radius.into();
    Self { center, radius }
  }
}

//

impl SphereMutTrait for Sphere
{

  #[ inline ]
  fn center_set( &mut self, src : &d2::V )
  {
    self.center = *src;
  }

  #[ inline ]
  fn radius_set( &mut self, src : S )
  {
    self.radius = src;
  }

}

//

// pub trait DerefMayby
// {
//   /// xxx?
//   type Target: ?Sized;
//   /// xxx?
//   fn deref( &self ) -> &Self::Target;
// }

// impl< T, D > DerefMayby for D
// where
//   D : Deref< Target = T >,
// {
//   type Target = T;
//   fn deref( &self ) -> &Self::Target
//   {
//     Deref::deref( self )
//   }
// }

// impl< T, D > DerefMayby for &D
// where
//   D : Deref< Target = T >,
// {
//   type Target = T;
//   fn deref( &self ) -> &Self::Target
//   {
//     Deref::deref( self )
//   }
// }

// pub trait From2< T > : Sized
// {
//   fn from2( value : T ) -> Self;
// }
//
// impl< T > From2< T > for T
// {
//   /// Returns the argument unchanged.
//   #[ inline( always ) ]
//   fn from2( t : T ) -> T
//   {
//     t
//   }
// }
//
// pub trait Into2< T > : Sized
// {
//   /// Converts this type into the (usually inferred) input type.
//   fn into2( self ) -> T;
// }
//
// impl< T, U > Into2< U > for T
// where
//   U : From2< T >,
// {
//   #[inline]
//   fn into2( self ) -> U
//   {
//     U::from2( self )
//   }
// }

// impl< 'a, T > From< &'a T > for Cow2< 'a, T >
// where
//   T : Clone
// {
//   fn from( src : &'a T ) -> Self
//   {
//     Cow2::Borrowed( src )
//   }
// }

// impl< 'a, T > From< < T as ToOwned >::Owned >
// for Cow2< 'a, T >
// where
//   T : Clone + ToOwned,
// {
//   fn from( src : < T as ToOwned >::Owned ) -> Self
//   {
//     Cow2::Owned( src )
//   }
// }

// /// Magic getting value from a refence.
// pub trait ValueOf< T > : From< T > + Into< T >
// {
//   /// Magic getting value from a refence.
//   fn val< Other : ValueOf< T > >( self ) -> Other
//   where
//     Self: Sized
//   {
//     Other::from( self.into() )
//   }
// }
//
// impl< T, FromInto > ValueOf< T > for FromInto
// where
//   FromInto : From< T > + Into< T >,
// {}
//
// //
//
// impl< 'a, 'b, Center, Radius > From< ( Center, Radius ) > for Sphere
// where
//   Center : ValueOf< d2::V >,
//   Radius : ValueOf< d2::S >,
// {
//   fn from( ( center, radius ) : ( Center, Radius ) ) -> Self
//   {
//     let center = center.val();
//     let radius = radius.val();
//     Self { center, radius }
//   }
// }

impl<> From< ( d2::V, d2::S ) >
for Sphere
{
  #[ inline ]
  fn from( ( center, radius ) : ( d2::V, d2::S ) ) -> Self
  {
    Self { center, radius }
  }
}

//

impl< Center, Radius > From< ( &Center, &Radius ) >
for Sphere
where
  Center : Clone + Into< d2::V >,
  Radius : Clone + Into< d2::S >,
{
  fn from( ( center, radius ) : ( &Center, &Radius ) ) -> Self
  {
    let center = center.clone().into();
    let radius = radius.clone().into();
    Self { center, radius }
  }
}

//

// impl< 'a, 'b, Center, Radius > From< ( Center, Radius ) > for Sphere
// where
//   Center : Into< Cow2< 'a, d2::V > >,
//   Radius : Into< Cow2< 'b, d2::S > >,
// {
//   fn from( ( center, radius ) : ( Center, Radius ) ) -> Self
//   {
//     let center = center.into().into_owned();
//     let radius = radius.into().into_owned();
//     Self { center, radius }
//   }
// }

//

#[cfg(test)]
mod tests
{
  // use super::*;
  // use diagnostics_tools::*;

}
