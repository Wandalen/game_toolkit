//!
//! Define interface for 2D box centered.
//!

// use crate::*;
use super::*;
// use d2::VExtTrait;
// use std::borrow::Cow;

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use super::CollideCboxTrait;
}

/// Ability of an entity to collide with a centered box.
pub trait CollideCboxTrait
{

  /// Collision with a solid centered box.
  fn cbox_collide_solid( &self, b : &impl CboxTrait ) -> Option< d2::Collision >;

  /// Collision with a hollow centered box.
  fn cbox_collide_hollow( &self, b : &impl CboxTrait ) -> Option< d2::Collision >;

}

impl CollideCboxTrait for Cbox
{

  /// Collision with a solid centered box.
fn cbox_collide_solid( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = [ self.to_intervals(), b.to_intervals() ];
    // println!( "m : {m:?}" );

    if
      ( m[ 1 ][ 0 ][ 0 ] < m[ 0 ][ 0 ][ 0 ] && m[ 1 ][ 0 ][ 1 ] < m[ 0 ][ 0 ][ 0 ] ) ||
      ( m[ 0 ][ 0 ][ 1 ] < m[ 1 ][ 0 ][ 0 ] && m[ 0 ][ 0 ][ 1 ] < m[ 1 ][ 0 ][ 1 ] )
    {
      return None;
    }
    if
      ( m[ 1 ][ 1 ][ 0 ] < m[ 0 ][ 1 ][ 0 ] && m[ 1 ][ 1 ][ 1 ] < m[ 0 ][ 1 ][ 0 ] ) ||
      ( m[ 0 ][ 1 ][ 1 ] < m[ 1 ][ 1 ][ 0 ] && m[ 0 ][ 1 ][ 1 ] < m[ 1 ][ 1 ][ 1 ] )
    {
      return None;
    }

    let ax_has_bx = m[ 0 ][ 0 ][ 0 ] <= m[ 1 ][ 0 ][ 0 ] && m[ 1 ][ 0 ][ 1 ] <= m[ 0 ][ 0 ][ 1 ];
    let bx_has_ax = m[ 1 ][ 0 ][ 0 ] <= m[ 0 ][ 0 ][ 0 ] && m[ 0 ][ 0 ][ 1 ] <= m[ 1 ][ 0 ][ 1 ];
    let ay_has_by = m[ 0 ][ 1 ][ 0 ] <= m[ 1 ][ 1 ][ 0 ] && m[ 1 ][ 1 ][ 1 ] <= m[ 0 ][ 1 ][ 1 ];
    let by_has_ay = m[ 1 ][ 1 ][ 0 ] <= m[ 0 ][ 1 ][ 0 ] && m[ 0 ][ 1 ][ 1 ] <= m[ 1 ][ 1 ][ 1 ];

    let ( apoint, bpoint ) = if ax_has_bx || bx_has_ax
    {
      if ay_has_by || by_has_ay
      {
        let d =
        [
          [ m[ 0 ][ 0 ][ 1 ] - m[ 1 ][ 0 ][ 0 ], m[ 0 ][ 0 ][ 0 ] - m[ 1 ][ 0 ][ 1 ] ],
          [ m[ 0 ][ 1 ][ 1 ] - m[ 1 ][ 1 ][ 0 ], m[ 0 ][ 1 ][ 0 ] - m[ 1 ][ 1 ][ 1 ] ],
        ];
        let min = [ d[ 0 ][ 0 ], d[ 0 ][ 1 ], d[ 1 ][ 0 ], d[ 1 ][ 1 ] ]
        .iter()
        // .iter().filter( | e | **e >= 0. )
        .copied()
        .min_by(| a, b | a.abs().partial_cmp( &b.abs() )
        .unwrap() )
        .unwrap();

        // println!( "d : {d:?} | min : {min}" );

        let order = if min == d[ 0 ][ 0 ]
        {
          ( 0, 1, 0, 1 )
        }
        else if min == d[ 0 ][ 1 ]
        {
          ( 0, 1, 1, 0 )
        }
        else if min == d[ 1 ][ 0 ]
        {
          ( 1, 0, 0, 1 )
        }
        else
        {
          ( 1, 0, 1, 0 )
        };

        let yy =
        [
          m[ 0 ][ order.1 ][ 0 ].max( m[ 1 ][ order.1 ][ 0 ] ),
          m[ 0 ][ order.1 ][ 1 ].min( m[ 1 ][ order.1 ][ 1 ] ),
        ];
        let y = ( yy[ 0 ] + yy[ 1 ] ) * 0.5;
        if order.0 == 0
        {
          let apoint = d2::V::new( m[ 1 ][ order.0 ][ order.2 ], y );
          let bpoint = d2::V::new( m[ 0 ][ order.0 ][ order.3 ], y );
          ( apoint, bpoint )
        }
        else
        {
          let apoint = d2::V::new( y, m[ 1 ][ order.0 ][ order.2 ] );
          let bpoint = d2::V::new( y, m[ 0 ][ order.0 ][ order.3 ] );
          ( apoint, bpoint )
        }

      }
      else
      {
        let xx = if ax_has_bx { m[ 1 ][ 0 ] } else { m[ 0 ][ 0 ] };
        let x = ( xx[ 0 ] + xx[ 1 ] ) * 0.5;
        let bhigher : usize = if m[ 0 ][ 1 ][ 0 ] < m[ 1 ][ 1 ][ 0 ] { 1 } else { 0 };
        let nbhigher = if bhigher == 0 { 1 } else { 0 };
        let bpoint = d2::V::new( x, m[ 0 ][ 1 ][ bhigher ] );
        let apoint = d2::V::new( x, m[ 1 ][ 1 ][ nbhigher ] );
        ( apoint, bpoint )
      }
    }
    else if ay_has_by || by_has_ay
    {
      // println!( "m : {m:?}" );
      let yy = if ay_has_by { m[ 1 ][ 1 ] } else { m[ 0 ][ 1 ] };
      let y = ( yy[ 0 ] + yy[ 1 ] ) * 0.5;
      let bhigher : usize = if m[ 0 ][ 0 ][ 0 ] < m[ 1 ][ 0 ][ 0 ] { 1 } else { 0 };
      let nbhigher = if bhigher == 0 { 1 } else { 0 };
      let bpoint = d2::V::new( m[ 0 ][ 0 ][ bhigher ], y );
      let apoint = d2::V::new( m[ 1 ][ 0 ][ nbhigher ], y );
      // println!( "yy : {yy:?} | apoint : {apoint:?} | bpoint : {bpoint:?}" );
      ( apoint, bpoint )
    }
    else
    {
      let xdir : usize = if m[ 0 ][ 0 ][ 0 ] < m[ 1 ][ 0 ][ 0 ] { 1 } else { 0 };
      let ydir : usize = if m[ 0 ][ 1 ][ 0 ] < m[ 1 ][ 1 ][ 0 ] { 1 } else { 0 };
      let nxdir = if xdir == 1 { 0 } else { 1 };
      let nydir = if ydir == 1 { 0 } else { 1 };
      let bpoint = d2::V::new( m[ 0 ][ 0 ][ xdir ], m[ 0 ][ 1 ][ ydir ] );
      let apoint = d2::V::new( m[ 1 ][ 0 ][ nxdir ], m[ 1 ][ 1 ][ nydir ] );
      if apoint[ 0 ] - bpoint[ 0 ] == 0. || apoint[ 1 ] - bpoint[ 1 ] == 0.
      {
        let point = ( apoint + bpoint ) * 0.5;
        ( point, point )
      }
      else
      {
        ( apoint, bpoint )
      }
    };

    // println!( "apoint : {apoint} | bpoint : {bpoint}" );
    let dir = bpoint - apoint;
    let depth = dir.length();
    let normal = dir / depth;
    let point = apoint + dir*0.5;
    return Some( d2::Collision { point, normal, depth } );
  }

  /// Collision with a hollow centered box.
  fn cbox_collide_hollow( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = [ self.to_intervals(), b.to_intervals() ];
    // println!( "m : {m:?}" );

    let mut d = [ [ 0. ; 2 ], [ 0. ; 2 ] ];
    d[ 0 ][ 0 ] = m[ 0 ][ 0 ][ 0 ] - m[ 1 ][ 0 ][ 0 ];
    d[ 0 ][ 1 ] = m[ 0 ][ 1 ][ 0 ] - m[ 1 ][ 1 ][ 0 ];
    d[ 1 ][ 0 ] = m[ 1 ][ 0 ][ 1 ] - m[ 0 ][ 0 ][ 1 ];
    d[ 1 ][ 1 ] = m[ 1 ][ 1 ][ 1 ] - m[ 0 ][ 1 ][ 1 ];
    // println!( "d : {d:?}" );

    if d[ 0 ][ 0 ] < 0. && d[ 0 ][ 1 ] < 0. && d[ 1 ][ 0 ] < 0. && d[ 1 ][ 1 ] < 0.
    {
      return None;
    }

    // d[ 0 ][ 0 ] = d[ 0 ][ 0 ].max( 0. );
    // d[ 0 ][ 1 ] = d[ 0 ][ 1 ].max( 0. );
    // d[ 1 ][ 0 ] = d[ 1 ][ 0 ].max( 0. );
    // d[ 1 ][ 1 ] = d[ 1 ][ 1 ].max( 0. );

    // let dir = bpoint - apoint;
    let dir = d2::V::from( d[ 0 ] ).max( d2::V::splat ( 0. ) ) - d2::V::from( d[ 1 ] ).max( d2::V::splat ( 0. ) );

    let cx = if dir.x < 0.0
    {
      0.5 * ( m[ 0 ][ 0 ][ 1 ] + m[ 1 ][ 0 ][ 0 ] )
    }
    else if dir.x > 0.0
    {
      0.5 * ( m[ 0 ][ 0 ][ 0 ] + m[ 1 ][ 0 ][ 1 ] )
    }
    else
    {
      if d[ 0 ][ 0 ] == 0.
      {
        m[ 1 ][ 0 ][ 0 ]
      }
      else if d[ 1 ][ 0 ] == 0.
      {
        m[ 1 ][ 0 ][ 1 ]
      }
      else
      {
        ( m[ 0 ][ 0 ][ 0 ].max( m[ 1 ][ 0 ][ 0 ] ) + m[ 0 ][ 0 ][ 1 ].min( m[ 1 ][ 0 ][ 1 ] ) ) * 0.5
      }
    };

    let cy = if dir.y < 0.0
    {
      0.5 * ( m[ 0 ][ 1 ][ 1 ] + m[ 1 ][ 1 ][ 0 ] )
    }
    else if dir.y > 0.0
    {
      0.5 * ( m[ 0 ][ 1 ][ 0 ] + m[ 1 ][ 1 ][ 1 ] )
    }
    else
    {
      if d[ 0 ][ 1 ] == 0.
      {
        m[ 1 ][ 1 ][ 0 ]
      }
      else if d[ 1 ][ 1 ] == 0.
      {
        m[ 1 ][ 1 ][ 1 ]
      }
      else
      {
       ( m[ 0 ][ 1 ][ 0 ].max( m[ 1 ][ 1 ][ 0 ] ) + m[ 0 ][ 1 ][ 1 ].min( m[ 1 ][ 1 ][ 1 ] ) ) * 0.5
      }
    };

    let depth = dir.length();
    let normal = dir / depth;
    // let point = apoint + dir*0.5;
    let point = d2::V::new( cx, cy );
    Some( d2::Collision { point, normal, depth } )
  }

}

impl CollideCboxTrait for d2::V
{

  /// Collision with a solid centered box.
  fn cbox_collide_solid( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = b.to_intervals();
    // println!( "m : {m:?}" );

    let mut d = [ [ 0. ; 2 ], [ 0. ; 2 ] ];
    d[ 0 ][ 0 ] = self[ 0 ] - m[ 0 ][ 0 ];
    d[ 0 ][ 1 ] = self[ 1 ] - m[ 1 ][ 0 ];
    d[ 1 ][ 0 ] = m[ 0 ][ 1 ] - self[ 0 ];
    d[ 1 ][ 1 ] = m[ 1 ][ 1 ] - self[ 1 ];

    if d[ 0 ][ 0 ] < 0. || d[ 1 ][ 0 ] < 0.
    {
      return None;
    }
    if d[ 0 ][ 1 ] < 0. || d[ 1 ][ 1 ] < 0.
    {
      return None;
    }

    // let dir = d2::V::from( d[ 0 ] ) - d2::V::from( d[ 1 ] );

    let min = [ d[ 0 ][ 0 ], d[ 0 ][ 1 ], d[ 1 ][ 0 ], d[ 1 ][ 1 ] ]
    // .iter()
    .iter().filter( | e | **e >= 0. )
    .copied()
    .min_by(| a, b | a.abs().partial_cmp( b ).unwrap() )
    .unwrap()
    ;

    // println!( "d : {d:?} | min : {min:?}" );

    let (  side, axis, side_sign ) = if min == d[ 0 ][ 0 ]
    {
      ( 0, 0, 1.0 )
    }
    else if min == d[ 0 ][ 1 ]
    {
      ( 0, 1, 1.0 )
    }
    else if min == d[ 1 ][ 0 ]
    {
      ( 1, 0, -1.0 )
    }
    else
    {
      ( 1, 1, -1.0 )
    };

    let mut dir = d2::V::new( 0., 0. );
    dir[ axis ] = side_sign * d[ side ][ axis ];
    let depth = dir.length();
    let normal = dir / depth;
    // let normal = if depth > Eps { dir / depth } else { dir };
    let point = self.clone();
    return Some( d2::Collision { point, normal, depth } );
  }

  /// Collision with a hollow centered box.
  fn cbox_collide_hollow( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = b.to_intervals();

    let mut d = [ [ 0. ; 2 ], [ 0. ; 2 ] ];
    d[ 0 ][ 0 ] = m[ 0 ][ 0 ] - self[ 0 ];
    d[ 0 ][ 1 ] = m[ 1 ][ 0 ] - self[ 1 ];
    d[ 1 ][ 0 ] = self[ 0 ] - m[ 0 ][ 1 ];
    d[ 1 ][ 1 ] = self[ 1 ] - m[ 1 ][ 1 ];

    if d[ 0 ][ 0 ] < 0. && d[ 1 ][ 0 ] < 0. && d[ 0 ][ 1 ] < 0. && d[ 1 ][ 1 ] < 0.
    {
      return None;
    }

    let min = [ d[ 0 ][ 0 ], d[ 0 ][ 1 ], d[ 1 ][ 0 ], d[ 1 ][ 1 ] ]
    .iter().filter( | e | **e >= 0. )
    .cloned()
    .min_by(| a, b | a.partial_cmp( b ).unwrap() )
    .unwrap()
    ;

    // println!( "m : {m:?} | d : {d:?} | min : {min:?}" );

    let ( side, axis, side_sign ) = if min == d[ 0 ][ 0 ]
    {
      ( 0, 0, 1. )
    }
    else if min == d[ 0 ][ 1 ]
    {
      ( 0, 1, 1. )
    }
    else if min == d[ 1 ][ 0 ]
    {
      ( 1, 0, -1. )
    }
    else
    {
      ( 1, 1, -1. )
    };

    let mut dir = d2::V::new( 0., 0. );
    dir[ axis ] = side_sign * d[ side ][ axis ];
    let depth = dir.length();
    let normal = dir / depth;
    let point = self.clone();
    return Some( d2::Collision { point, normal, depth } );
  }

}

//

use crate::d2::sphere::Sphere;
impl CollideCboxTrait for Sphere
{

  /// Collision with a solid centered box.
  fn cbox_collide_solid( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = b.to_intervals();
    // println!( "m : {m:?}" );

    let mut d = [ [ 0. ; 2 ], [ 0. ; 2 ] ];
    d[ 0 ][ 0 ] = self.center[ 0 ] - m[ 0 ][ 0 ] + self.radius;
    d[ 0 ][ 1 ] = self.center[ 1 ] - m[ 1 ][ 0 ] + self.radius;
    d[ 1 ][ 0 ] = m[ 0 ][ 1 ] - self.center[ 0 ] + self.radius;
    d[ 1 ][ 1 ] = m[ 1 ][ 1 ] - self.center[ 1 ] + self.radius;

    if d[ 0 ][ 0 ] < 0. || d[ 1 ][ 0 ] < 0.
    {
      return None;
    }
    if d[ 0 ][ 1 ] < 0. || d[ 1 ][ 1 ] < 0.
    {
      return None;
    }

    let min = [ d[ 0 ][ 0 ], d[ 0 ][ 1 ], d[ 1 ][ 0 ], d[ 1 ][ 1 ] ]
    // .iter()
    .iter().filter( | e | **e >= 0. )
    .copied()
    // .map( | e | e.abs() )
    .min_by(| a, b | a.partial_cmp( &b ).unwrap() )
    // .min_by(| a, b | a.abs().partial_cmp( &b.abs() ).unwrap() )
    .unwrap()
    ;

    // println!( "d : {d:?} | min : {min:?}" );

    let ( side, axis, side_sign ) = if min == d[ 0 ][ 0 ]
    {
      ( 0, 0, 1. )
    }
    else if min == d[ 0 ][ 1 ]
    {
      ( 0, 1, 1. )
    }
    else if min == d[ 1 ][ 0 ]
    {
      ( 1, 0, -1. )
    }
    else
    {
      ( 1, 1, -1. )
    };

    let mut dir = d2::V::new( 0., 0. );
    dir[ axis ] = side_sign * d[ side ][ axis ];
    let depth = dir.length();
    let normal = dir / depth;
    // let normal = if depth > Eps { dir / depth } else { dir };
    // xxx : remove eps?
    let point = self.center.clone();
    return Some( d2::Collision { point, normal, depth } );
  }

  /// Collision with a hollow centered box.
  fn cbox_collide_hollow( &self, b : &impl CboxTrait ) -> Option< d2::Collision >
  {
    let m = b.to_intervals();

    let mut d = [ [ 0. ; 2 ], [ 0. ; 2 ] ];
    d[ 0 ][ 0 ] = m[ 0 ][ 0 ] - self.center[ 0 ] + self.radius;
    d[ 0 ][ 1 ] = m[ 1 ][ 0 ] - self.center[ 1 ] + self.radius;
    d[ 1 ][ 0 ] = self.center[ 0 ] - m[ 0 ][ 1 ] + self.radius;
    d[ 1 ][ 1 ] = self.center[ 1 ] - m[ 1 ][ 1 ] + self.radius;

    // println!( "m : {m:?} | d : {d:?}" );

    if d[ 0 ][ 0 ] < 0. && d[ 1 ][ 0 ] < 0. && d[ 0 ][ 1 ] < 0. && d[ 1 ][ 1 ] < 0.
    {
      return None;
    }

    d[ 0 ][ 0 ] = d[ 0 ][ 0 ].max( 0.0 );
    d[ 0 ][ 1 ] = d[ 0 ][ 1 ].max( 0.0 );
    d[ 1 ][ 0 ] = d[ 1 ][ 0 ].max( 0.0 );
    d[ 1 ][ 1 ] = d[ 1 ][ 1 ].max( 0.0 );

    let dir = d2::V::from( d[ 1 ] ) - d2::V::from( d[ 0 ] );
    let depth = dir.length();
    let normal = dir / depth;
    let point = self.center.clone();
    return Some( d2::Collision { point, normal, depth } );
  }

}

//

#[cfg(test)]
mod tests
{
  use super::*;
  use diagnostics_tools::*;
  #[ allow( unused_imports ) ]
  use impls_index::prelude::*;

  // tests_impls!
  // {

  #[ test ]
  fn point_cbox_collide_solid_test()
  {

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |     + *
    //    *       |(1.5,1)*
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 1.5, 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 1. ),
      normal : d2::V::new( -1., 0.),
      depth : 0.5,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( -1.5, 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 1. ),
      normal : d2::V::new( 1., 0.),
      depth : 0.5,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1.0, 1.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 1.5 ),
      normal : d2::V::new( 0., -1.),
      depth : 0.5,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1.0, -1.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -1.5 ),
      normal : d2::V::new( 0., 1.),
      depth : 0.5,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *  (3,1)
    //    *       |       *   +
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 3.0, 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    let a = d2::V::new( -3.0, 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    let a = d2::V::new( 1.0, 3.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    let a = d2::V::new( 1.0, -3.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |       + (2,1)
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 2., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( -2., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -2., 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., 2. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 2. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., -2. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -2. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

  }

  #[ test ]
  fn point_cbox_collide_hollow_test()
  {

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |   +   *
    //    *       | (1,1) *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 1., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::V::new( -1., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., -1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::V::new( -1., -1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |       *   +
    //    *       |       *  (3,1)
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 3., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 3., 1. ),
      normal : d2::V::new( -1., 0.),
      depth : 1.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( -3., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -3., 1. ),
      normal : d2::V::new( 1., 0.),
      depth : 1.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., 3. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 3. ),
      normal : d2::V::new( 0., -1.),
      depth : 1.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., -3. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -3. ),
      normal : d2::V::new( 0., 1.),
      depth : 1.0,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |       + (2,1)
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::V::new( 2., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( -2., 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -2., 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., 2. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 2. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::V::new( 1., -2. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -2. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

  }

  #[ test ]
  fn sphere_cbox_collide_solid_test()
  {
    use crate::d2::sphere::SphereTrait;

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       | (   + * )
    //    *       |(1.5,1)*
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1.5, 1.0 ), 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 1. ),
      normal : d2::V::new( -1., 0. ),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -1.5, 1.0 ), 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 1. ),
      normal : d2::V::new( 1., 0. ),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1.0, 1.5 ), 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 1.5 ),
      normal : d2::V::new( 0., -1. ),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1.0, -1.5 ), 1.0 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -1.5 ),
      normal : d2::V::new( 0., 1. ),
      depth : 1.5,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       * (3,1)
    //    *       |       * ( + )
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 3., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -3., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 3. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., -3. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_solid( &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       * (3,1)
    //    *       |       (   +   )
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 3., 1. ), 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 3., 1. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -3., 1. ), 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -3., 1. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 3. ), 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 3. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., -3. ), 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_solid( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -3. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

  }

  #[ test ]
  fn sphere_cbox_collide_hollow_test()
  {
    use crate::d2::sphere::SphereTrait;

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       | ( + ) *
    //    *       | (1,1) *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -1., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., -1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -1., -1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |       * ( + )
    //    *       |       *  (3,1)
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 3., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 3., 1. ),
      normal : d2::V::new( 1., 0.),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -3., 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -3., 1. ),
      normal : d2::V::new( -1., 0.),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 3. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 3. ),
      normal : d2::V::new( 0., 1.),
      depth : 1.5,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., -3. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let got = a.cbox_collide_hollow( &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -3. ),
      normal : d2::V::new( 0., -1.),
      depth : 1.5,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |     (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |   ( + )
    //    *       |(1.5,1)*
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1.5, 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( -1.5, 1. ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 1. ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 1.5 ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 1.5 ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., -1.5 ), 0.5 );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., -1.5 ),
      normal : d2::V::new( 0., 0.),
      depth : 0.0,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |        (2,2)
    //    * * * * | * * * *
    //    *       (   +   )
    //    *       | (1,1.5)
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = d2::Sphere::FromCenterAndRadius( ( 1., 1.5 ), 1. );
    let b = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let mut got = a.cbox_collide_hollow( &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1., 1.5 ),
      normal : d2::V::new( 0., 0.),
      depth : 0.5,
    });
    a_id!( got , exp );

    //

  }

  #[ test ]
  fn cbox_cbox_collide_solid_test()
  {

    //
    //            ^
    //            |              (4,3)
    //            |           + + +
    //            |     (2,2) +   +
    //    * * * * | * * * *   + + +
    //    *       |       * (3,2)
    //    *       |       *
    //    *       |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 3., 4. ], [ 2., 3. ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |                 (4,3)
    //            |   + + + + + + +
    //            |   +  (2,2)    +
    //    * * * * | * + * *       +
    //    *       |   +   *       +
    //    *       |   + + + + + + +
    //    *       | (1,1) *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 2.5, 2.0 ), d2::V::new( 3.0, 2.0 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ 1., 4. ], [ 1., 3. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 1.5 ),
      normal : d2::V::new( 1.0 / 2_f32.sqrt(), 1.0 / 2_f32.sqrt() ),
      depth : 2_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( -2.5, 2.0 ), d2::V::new( 3.0, 2.0 ) );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 1.5 ),
      normal : d2::V::new( -1.0 / 2_f32.sqrt(), 1.0 / 2_f32.sqrt() ),
      depth : 2_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 2.5, -2.0 ), d2::V::new( 3.0, 2.0 ) );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, -1.5 ),
      normal : d2::V::new( 1.0 / 2_f32.sqrt(), -1.0 / 2_f32.sqrt() ),
      depth : 2_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( -2.5, -2.0 ), d2::V::new( 3.0, 2.0 ) );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, -1.5 ),
      normal : d2::V::new( -1.0 / 2_f32.sqrt(), -1.0 / 2_f32.sqrt() ),
      depth : 2_f32.sqrt(),
    });
    a_id!( got , exp );

    //
    //            ^
    //            |                 (4,3)
    //            |       + + + + +
    //            |      (2,2)    +
    //    * * * * | * * * +       +
    //    *       |       +       +
    //    *       |       + + + + +
    //    *       | (2,1) *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 2., 4. ], [ 1., 3. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 1.5 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -4., -2. ], [ 1., 3. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -2., 1.5 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 3. ], [ 2., 4. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 3. ], [ -4., -2. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, -2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //
    //            ^
    //        + + | + + + (1.5,4)
    //        +   |     +
    //        +   |     +
    //        +   |     +  (2,2)
    //    * * + * | * * + *
    //    *   +   |     + *
    //    *   + + | + + + *
    //    *(-1,1) |       *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 0.25, 2.5 ), d2::V::new( 2.5, 3.0 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 1.5 ], [ 1., 4. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, 1.5 ),
      normal : d2::V::new( 0.0, 1.0 ),
      depth : 1.0,
    });
    a_id!( got, exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 0.25, -2.5 ), d2::V::new( 2.5, 3.0 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 1.5 ], [ -4., -1. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, -1.5 ),
      normal : d2::V::new( 0.0, -1.0 ),
      depth : 1.0,
    });
    a_id!( got, exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 2.5, 0.25 ), d2::V::new( 3.0, 2.5 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ 1., 4. ], [ -1., 1.5 ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 0.25 ),
      normal : d2::V::new( 1.0, 0.0 ),
      depth : 1.0,
    });
    a_id!( got, exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( -2.5, 0.25 ), d2::V::new( 3.0, 2.5 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ -4., -1. ], [ -1., 1.5 ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 0.25 ),
      normal : d2::V::new( -1.0, 0.0 ),
      depth : 1.0,
    });
    a_id!( got, exp );

    //
    //            ^
    //        + + | + + + (1.5,4)
    //        +   |     +
    //        +   |     +
    //        +   |     +  (2,2)
    //    * * + * | * * + *
    //    *   +   |     + *
    //    *   +   |     + *
    //    *   +   |     + *
    //  ----------|------------------>
    //    *   +   |     + *
    //    *   +   |     + *
    //    *   +   |     + *
    //    * * + * | * * + *
    //(-2,-2) +   |     +
    //        + + | + + +
    //     (-1,-3)
    //
    //   (3,0)->

    // d : [[-3.0, 3.5], [-5.0, 6.0]] | min : -3

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( 0.25, 0.5 ), d2::V::new( 2.5, 7.0 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 1.5 ], [ -3., 4. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.5, 0.0 ),
      normal : d2::V::new( 1.0, 0.0 ),
      depth : 3.0,
    });
    a_id!( got, exp );

    //
    //            ^
    //        + + | + (0.5,4)
    //        +   | +
    //        +   | +
    //        +   | +      (2,2)
    //    * * + * | + * * *
    //    *   +   | +     *
    //    *   +   | +     *
    //    *   +   | +     *
    //  ----------|------------------>
    //    *   +   | +     *
    //    *   +   | +     *
    //    *   +   | +     *
    //    * * + * | + * * *
    //(-2,-2) +   | +
    //        + + | +
    //     (-1,-3)
    //
    //   <-(2.5,0)

    // d : [[-3.0, 2.5], [-5.0, 6.0]] | min : 2.5

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromCenterAndSize( d2::V::new( -0.25, 0.5 ), d2::V::new( 1.5, 7.0 ) );
    a_id!( a.to_intervals(), [ [ -2., 2. ], [ -2., 2. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 0.5 ], [ -3., 4. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -0.75, 0.0 ),
      normal : d2::V::new( -1.0, 0.0 ),
      depth : 2.5,
    });
    a_id!( got, exp );

    //
    //            ^
    //            + + | + + (1,3)
    //            +   |   +
    //            +   |   +
    //            +   |   +      (3,1)
    //    * * * * + * | * + * * * *
    //    *       +   |   +       *
    //   -------------|------------------>
    //    *       +   |   +       *
    //    * * * * + * | * + * * * *
    // (-3,-1)    +   |   +
    //            + + | + +
    //         (-1,-2)
    //
    //   (3,0)->

    // d : [[-4.0, 4.0], [-3.0, 4.0]] | min : -3

    let a = Cbox::FromCenterAndSize( d2::V::new( 0.0, 0.0 ), d2::V::new( 6.0, 2.0 ) );
    let b = Cbox::FromCenterAndSize( d2::V::new( 0.0, 0.5 ), d2::V::new( 2.0, 5.0 ) );
    a_id!( a.to_intervals(), [ [ -3., 3. ], [ -1., 1. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 1. ], [ -2., 3. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.0, -0.5 ),
      normal : d2::V::new( 0.0, 1.0 ),
      depth : 3.0,
    });
    a_id!( got, exp );

    //
    //            ^
    //            + + | + + (1,2)
    //            +   |   +      (3,1)
    //    * * * * + * | * + * * * *
    //    *       +   |   +       *
    //   -------------|------------------>
    //    *       +   |   +       *
    //    * * * * + * | * + * * * *
    // (-3,-1)    +   |   +
    //            +   |   +
    //            +   |   +
    //            + + | + +
    //         (-1,-3)
    //
    //   (3,0)->

    // d : [[-4.0, 4.0], [-4.0, 3.0]] | min : 3

    let a = Cbox::FromCenterAndSize( d2::V::new( 0.0, 0.0 ), d2::V::new( 6.0, 2.0 ) );
    let b = Cbox::FromCenterAndSize( d2::V::new( 0.0, -0.5 ), d2::V::new( 2.0, 5.0 ) );
    a_id!( a.to_intervals(), [ [ -3., 3. ], [ -1., 1. ] ] );
    a_id!( b.to_intervals(), [ [ -1., 1. ], [ -3., 2. ] ] );
    let got = Cbox::cbox_collide_solid( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.0, 0.5 ),
      normal : d2::V::new( 0.0, -1.0 ),
      depth : 3.0,
    });
    a_id!( got, exp );

    //
    //            ^
    //            |
    //            |
    //            |      (2,2)
    //    * * * * | * * * +
    //    *       |       +      (4,1)
    //    *       |       + + + + +
    //    *       |       +       +
    //  ----------|------------------>
    //    *       |       + + + + +
    //    *       |       * (2,-0.5)
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 2., 4. ], [ -0.5, 1. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 0.25 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -4., -2. ], [ -0.5, 1. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -2., 0.25 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals(  [ -0.5, 1. ], [ 2., 4. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, 2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals(  [ -0.5, 1. ], [ -4., -2. ] );
    let mut got = Cbox::cbox_collide_solid( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, -2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

  }

  #[ test ]
  fn cbox_cbox_collide_hollow_test()
  {

    //
    //            ^
    //            |                 (4,3)
    //            |   + + + + + + +
    //            |   +  (2,2)    +
    //    * * * * | * + * *       +
    //    *       |   +   *       +
    //    *       |   + + + + + + +
    //    *       | (1,1) *
    //  ----------|------------------>
    //    *       |       *
    //    *       |       *
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //
    // d : [[0.0, 0.0], [2.0, 1.0]]

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 4. ], [ 1., 3. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 1.5 ),
      normal : d2::V::new( -2.0 / 5_f32.sqrt(), -1.0 / 5_f32.sqrt() ),
      depth : 5_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -4., -1. ], [ 1., 3. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 1.5 ),
      normal : d2::V::new( 2.0 / 5_f32.sqrt(), -1.0 / 5_f32.sqrt() ),
      depth : 5_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 4. ], [ -3., -1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, -1.5 ),
      normal : d2::V::new( -2.0 / 5_f32.sqrt(), 1.0 / 5_f32.sqrt() ),
      depth : 5_f32.sqrt(),
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -4., -1. ], [ -3., -1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, -1.5 ),
      normal : d2::V::new( 2.0 / 5_f32.sqrt(), 1.0 / 5_f32.sqrt() ),
      depth : 5_f32.sqrt(),
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |   (2,2)
    //    * * * * | * * * *
    //    *       |       *       (4,1)
    //    *       |   + + + + + + +
    //    *       |   +           +
    //  ----------|------------------>
    //    *       |   + + + + + + +
    //    *       |(1,-.5)*
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //
    // d : [[0.0, 0.0], [2.0, 1.0]]

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 4. ], [ -0.5, 1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 1.5, 0.25 ),
      normal : d2::V::new( -1.0, 0.0 ),
      depth : 2.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -4., -1. ], [ -0.5, 1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -1.5, 0.25 ),
      normal : d2::V::new( 1.0, 0.0 ),
      depth : 2.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -0.5, 1. ], [ 1., 4. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, 1.5 ),
      normal : d2::V::new( 0.0, -1.0 ),
      depth : 2.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -0.5, 1. ], [ -4., -1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, -1.5 ),
      normal : d2::V::new( 0.0, 1.0 ),
      depth : 2.,
    });
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |   (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |   + + * (1.5,1)
    //    *       |   + + *
    //  ----------|------------------>
    //    *       |   + + *
    //    *       |(1,-.5)*
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //
    // d : [[0.0, 0.0], [2.0, 1.0]]

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 1.5 ], [ -0.5, 1. ] );
    let got = Cbox::cbox_collide_hollow( &a, &b );
    let exp = None;
    a_id!( got , exp );

    //
    //            ^
    //            |
    //            |
    //            |   (2,2)
    //    * * * * | * * * *
    //    *       |       *
    //    *       |   + + + (2,1)
    //    *       |   +   +
    //  ----------|------------------>
    //    *       |   + + +
    //    *       |(1,-.5)*
    //    *       |       *
    //    * * * * | * * * *
    // (-2,-2)    |
    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ 1., 2. ], [ -0.5, 1. ] );
    let mut got = Cbox::cbox_collide_hollow( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 2., 0.25 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -2., -1. ], [ -0.5, 1. ] );
    let mut got = Cbox::cbox_collide_hollow( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( -2., 0.25 ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -0.5, 1. ], [ 1., 2. ] );
    let mut got = Cbox::cbox_collide_hollow( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, 2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

    let a = Cbox::FromIntervals( [ -2., 2. ], [ -2., 2. ] );
    let b = Cbox::FromIntervals( [ -0.5, 1. ], [ -2., -1. ] );
    let mut got = Cbox::cbox_collide_hollow( &a, &b );
    if let Some( ref mut collision ) = got
    {
      collision.normal = d2::V::splat( 0. );
    }
    let exp = Some( d2::Collision
    {
      point : d2::V::new( 0.25, -2. ),
      normal : d2::V::new( 0., 0. ),
      depth : 0.,
    });
    a_id!( got , exp );

    //

  }

/*
  tests_index!
  {
    point_cbox_collide_solid_test,
    point_cbox_collide_hollow_test,
    sphere_cbox_collide_solid_test,
    sphere_cbox_collide_hollow_test,
    cbox_cbox_collide_solid_test,
    cbox_cbox_collide_hollow_test,
  }
*/

}
