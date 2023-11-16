//!
//! Define interface for 2D box centered.
//!

use super::*;
// use d2::VExtTrait;
// use std::borrow::Cow;

/// Namespace to include with asterisk.
pub mod prelude
{
  // use crate::*;
  // pub use bevy_math::prelude::*;
  // pub use super::SphereTrait;
  // pub use super::SphereMutTrait;
  // pub use super::CollideSphereTrait;
}

/// Ability of an entity to collide with a sphere.
pub trait CollideSphereTrait
{

  /// Collision with a solid sphere 2D.
  fn sphere_collide_solid( &self, b : &impl SphereTrait ) -> Option< d2::Collision >;

  /// Collision with a hollow sphere 2D.
  fn sphere_collide_hollow( &self, b : &impl SphereTrait ) -> Option< d2::Collision >;

}

//

impl CollideSphereTrait for Sphere
{

  fn sphere_collide_solid( &self, b : &impl SphereTrait ) -> Option< d2::Collision >
  {
    let r = self.radius + b.radius();
    let dir = b.center() - self.center;
    let mdir2 = dir.length_squared();
    // println!( "dir : {dir} | r : {r} | mdir2 : {mdir2}" );
    if r*r < mdir2
    {
      return None;
    }
    let mdir = mdir2.sqrt();
    let depth = r - mdir;
    let normal = dir / mdir;
    let point = self.center + dir * ( self.radius / r );
    Some( d2::Collision { point, normal, depth } )
  }

  //

  fn sphere_collide_hollow( &self, b : &impl SphereTrait ) -> Option< d2::Collision >
  {
    // let r = self.radius + b.radius();
    let dir = b.center() - self.center;
    let mdir = dir.length();
    let depth =  mdir + self.radius - b.radius();
    // println!( "a : {self:?} | b : {:?}", b.to_sphere() );
    // println!( "dir : {dir} | r : {r} | mdir : {mdir}" );
    if depth < 0.
    {
      return None;
    }
    // let depth = mdir;
    let normal = dir / mdir;
    let apoint = self.center + normal * self.radius;
    let bpoint = b.center() - normal * b.radius();
    let point = ( apoint + bpoint ) * 0.5;
    Some( d2::Collision { point, normal, depth } )
  }

}

//

impl CollideSphereTrait for d2::V
{

  fn sphere_collide_solid( &self, b : &impl SphereTrait ) -> Option< d2::Collision >
  {
    let r = b.radius();
    let dir = b.center() - *self;
    let mdir2 = dir.length_squared();
    // println!( "dir : {dir} | r : {r} | mdir2 : {mdir2}" );
    if r*r < mdir2
    {
      return None;
    }
    let mdir = mdir2.sqrt();
    let depth = r - mdir;
    let normal = dir / mdir;
    let point = *self + dir * 0.5;
    Some( d2::Collision { point, normal, depth } )
  }

  //

  fn sphere_collide_hollow( &self, b : &impl SphereTrait ) -> Option< d2::Collision >
  {
    // let r = b.radius();
    let dir = b.center() - *self;
    let mdir = dir.length();
    let depth =  mdir - b.radius();
    // println!( "a : {self:?} | b : {:?}", b.to_sphere() );
    // println!( "dir : {dir} | r : {r} | mdir : {mdir}" );
    if depth < 0.
    {
      return None;
    }
    // let depth = mdir;
    let normal = dir / mdir;
    let apoint = *self;
    let bpoint = b.center() - normal * b.radius();
    let point = ( apoint + bpoint ) * 0.5;
    Some( d2::Collision { point, normal, depth } )
  }

}

//

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use diagnostics_tools::*;

  #[ test ]
  fn sphere_sphere_collide_solid_test()
  {

    //            ^
    //            |
    //     (-1,0) | (1,0)
    // -(-----*-(-|-)-*-----)--------->
    //            |
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( -1., 0. ), 1.5 );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 1., 0. ), 1.5 );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0., 0. ),
      normal : d2::V::new( 1., 0. ),
      depth : 1.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //     (-1,0) | (1,0)
    //  --(---*---)---*-------)------->
    //        (   |           )
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( -1., 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 1., 0. ), 2.0 );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -0.3333333, 0. ),
      normal : d2::V::new( 1., 0.),
      depth : 1.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //     (-1,0) | (1,0)
    //  --(---*---|---*---)----------->
    //            |
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( -1., 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 1., 0. ), 1. );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::splat( 0. ),
      normal : d2::V::new( 1., 0. ),
      depth : 0.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //        ^   (   *   )
    //            | (1,1)
    // -----------|------------------->
    //    (-1,-1) |
    // (      +   |   )
    //            |
    //        ~   |

    // xxx : use a_eq in these tests
    let a = Sphere::FromCenterAndRadius( d2::V::new( -1., -1. ), 2. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 1., 1. ), 1. );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::splat( 0.33333337 ),
      normal : d2::V::new( 2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2. ),
      depth : 3. - 2.*2.0_f32.sqrt(),
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //     (-1,0) |     (2,0)
    //  --(---*---)---(---*---)------->
    //            |
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( -1., 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 2., 0. ), 1. );
    let got = a.sphere_collide_solid( &b );
    let exp = None;
    a_id!( got ,exp );

    //

  }

  #[ test ]
  fn sphere_sphere_collide_hollow_test()
  {

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------+---(---*---)----->
    //            |
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( 2., 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 1.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------+-(---*-)-)--------->
    //            |    (1.5,0)
    //            |

    let a = Sphere::FromCenterAndRadius( d2::V::new( 1.5, 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.25, 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 0.5,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------(---*---)--------->
    //            | (1,0)
    //            |

    // let a = d2::V::new( 1., 0. );
    let a = Sphere::FromCenterAndRadius( d2::V::new( 1., 0. ), 1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.0, 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 0.0,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------(-*-)-----)--------->
    //            |(0.5,0)
    //            |

    // let a = d2::V::new( 0.5, 0. );
    let a = Sphere::FromCenterAndRadius( d2::V::new( 0.5, 0. ), 0.5 );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = None;
    a_id!( got ,exp );

    //

  }

  #[ test ]
  fn point_sphere_collide_solid_test()
  {

    //            ^
    //            |
    // (-2,0)     | (1,0)  (2,0)
    // ---(-------+---*---)----------->
    //            |
    //            |

    let a = d2::V::new( 1., 0. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.5, 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 1.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //(           |   *           )
    //            | (1,1)
    // -----------|------------------->
    //    (-1,-1) |
    //        +   |
    //            |
    //            |

    let a = d2::V::new( -1., -1. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 1., 1. ), 3. );
    let got = a.sphere_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::splat( 0. ),
      normal : d2::V::new( 2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2. ),
      depth : 3. - 2.*2.0_f32.sqrt(),
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //     (-1,0) |     (2,0)
    //  ------*---|---(---*---)------->
    //            |
    //            |

    let a = d2::V::new( -1., 0. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 2., 0. ), 1. );
    let got = a.sphere_collide_solid( &b );
    let exp = None;
    a_id!( got ,exp );

    //

  }

  #[ test ]
  fn point_sphere_collide_hollow_test()
  {

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------+-------*--------->
    //            |
    //            |

    let a = d2::V::new( 2., 0. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 0.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------+-------)---*------->
    //            |         (3,0)
    //            |

    let a = d2::V::new( 3., 0. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2.5, 0. ),
      normal : d2::V::new( -1., 0. ),
      depth : 1.,
    });
    a_id!( got ,exp );

    //            ^
    //            |
    //            |
    //            |
    //   (-2,0)   |      (2,0)
    //  --(-------|-*-------)--------->
    //            |(0.5,0)
    //            |

    let a = d2::V::new( 0.5, 0. );
    let b = Sphere::FromCenterAndRadius( d2::V::new( 0., 0. ), 2. );
    let got = a.sphere_collide_hollow( &b );
    let exp = None;
    a_id!( got ,exp );

    //

  }

/*
  tests_index!
  {
    sphere_sphere_collide_solid_test,
    sphere_sphere_collide_hollow_test,
    point_sphere_collide_solid_test,
    point_sphere_collide_hollow_test,
  }
*/

}
