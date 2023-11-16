//!
//! Define interface for 2D vector.
//!

use crate::*;
use rand::prelude::*;
use sealed::sealed;

/// Namespace to include with asterisk.
pub mod prelude
{
  // pub use bevy_math::prelude::*;
  pub use super::VExtTrait;
}

/// Alias for 2D vector.
pub type V = Vec2;
pub use crate::S;

///

#[ sealed ]
pub trait VExtTrait
{
  /// Returns the square root.
  fn sqrt( &self ) -> Self;
  /// Square root inplace.
  fn sqrt_inplace( &mut self );

  /// Normaliz itself to length 1.0. Do the operation inplace not creating a new instance.
  fn normalize_inplace( &mut self );
  /// Returns a vector with a length no less than `a` and no more than `b`. Do the operation inplace not creating a new instance.
  fn clamp_inplace( &mut self, a : &Self, b : &Self );
  /// Construct rundom value. Each element of the vector is in range [ 0 .. 1 ].
  fn Random() -> V;
  // xxx : add seed
}

#[ sealed ]
impl VExtTrait for V
{
  #[ inline( always ) ]
  fn sqrt( &self ) -> Self
  {
    Self { x : self.x.sqrt(), y : self.y.sqrt() }
  }
  #[ inline( always ) ]
  fn sqrt_inplace( &mut self )
  {
    self.x = self.x.sqrt();
    self.y = self.y.sqrt();
  }
  #[ inline( always ) ]
  fn normalize_inplace( &mut self )
  {
    *self = self.normalize();
  }
  #[ inline( always ) ]
  fn clamp_inplace( &mut self, a : &Self, b : &Self )
  {
    *self = self.clamp( *a, *b );
  }
  fn Random() -> V
  {
    let mut rng = rand::thread_rng();
    V::new( rng.gen::< S >(), rng.gen::< S >() )
  }
}
