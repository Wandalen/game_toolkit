//!
//! Describe movable objects.
//!

use crate::*;
use math::d2;
use math::d2::prelude::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

/// Interface of a sphere for purpose of position resolution after collision.
pub trait SphereCollisionPositonResolutionTrait
{
  /// Position.
  fn pos( &mut self ) -> &mut d2::V;
  /// Radius.
  fn radius( &self ) -> d2::S;
  /// Inverse mass.
  fn inv_mass( &self ) -> d2::S;
}

/// Collide two spheres.
pub fn sphere_sphere_collide
(
  // a : &mut impl SphereCollisionPositonResolutionTrait,
  // b : &mut impl SphereCollisionPositonResolutionTrait,
  a_pos : &mut d2::V,
  a_radius : d2::S,
  a_inv_mass : d2::S,
  b_pos : &mut d2::V,
  b_radius : d2::S,
  b_inv_mass : d2::S,
) -> Option< d2::Collision >
{
  // let some_collision = d2::Sphere::sphere_collide_solid
  // (
  //   &d2::Sphere::FromCenterAndRadius( *a.pos(), a.radius() ),
  //   &d2::Sphere::FromCenterAndRadius( *b.pos(), b.radius() ),
  // );

  let some_collision = d2::Sphere::sphere_collide_solid
  (
    &d2::Sphere::FromCenterAndRadius( *a_pos, a_radius ),
    &d2::Sphere::FromCenterAndRadius( *b_pos, b_radius ),
  );

  if let Some( collision ) = some_collision
  {
    collision_position_solve
    (
      &collision,
      a_pos,
      a_inv_mass,
      // a_restitution,
      b_pos,
      b_inv_mass,
      // b_restitution,
    );
    // collision_position_solve
    // (
    //   &collision,
    //   a.pos(),
    //   a.inv_mass(),
    //   // a_restitution,
    //   b.pos(),
    //   b.inv_mass(),
    //   // b_restitution,
    // );
    // collision_position_solve
    // (
    //   &collision,
    //   a,
    //   b,
    // );
    if collision.depth > 0.
    {
      return Some( collision );
    }
  }

  None
}

/// Interface of a sphere for purpose of position resolution after collision.
pub trait BoxCollisionPositonResolutionTrait
{
  /// Position.
  fn pos( &mut self ) -> &mut d2::V;
  /// Radius.
  fn size( &self ) -> &d2::V;
  /// Inverse mass.
  fn inv_mass( &self ) -> d2::S;
}

// /// Collide a sphere and a box.
// pub fn shpere_box_collide
// (
//   a : &mut impl SphereCollisionPositonResolutionTrait,
//   b : &mut impl BoxCollisionPositonResolutionTrait,
//   // a_pos : &mut d2::V,
//   // a_radius : d2::S,
//   // a_inv_mass : d2::S,
//   // // a_restitution : d2::S,
//   // b_pos : &mut d2::V,
//   // b_size : &d2::V,
//   // b_inv_mass : d2::S,
//   // // b_resititution : d2::S,
// ) -> Option< d2::Collision >
// {
//   // let some_collision = self.sphere().cbox_collide_hollow( &playground.cbox );
//
//   let some_collision = d2::Sphere::cbox_collide_hollow
//   (
//     &d2::Sphere::FromCenterAndRadius( *a.pos(), a.radius() ),
//     &d2::Cbox::FromCenterAndSize( *b.pos(), *b.size() ),
//   );
//
//   if let Some( collision ) = some_collision
//   {
//     // let mut center = b_pos;
//     collision_position_solve
//     (
//       &collision,
//       a,
//       b,
//       // a_pos,
//       // a_inv_mass,
//       // a_restitution,
//       // b_pos,
//       // b_inv_mass,
//       // b_restitution,
//     );
//     if collision.depth > 0.
//     {
//       return Some( collision );
//     }
//   }
//
//   return None;
// }

/// Interface of a sphere for purpose of position resolution after collision.
pub trait CollisionPositonResolutionTrait
{
  /// Position.
  fn pos( &mut self ) -> &mut d2::V;
  /// Inverse mass.
  fn inv_mass( &self ) -> d2::S;
}

impl< T > CollisionPositonResolutionTrait for T
where
  T : SphereCollisionPositonResolutionTrait
{
  /// Position.
  #[ inline( always ) ]
  fn pos( &mut self ) -> &mut d2::V
  {
    SphereCollisionPositonResolutionTrait::pos( self )
  }
  /// Inverse mass.
  #[ inline( always ) ]
  fn inv_mass( &self ) -> d2::S
  {
    SphereCollisionPositonResolutionTrait::inv_mass( self )
  }
}

// impl< T > CollisionPositonResolutionTrait for T
// where
//   T : BoxCollisionPositonResolutionTrait
// {
//   /// Position.
//   #[ inline( always ) ]
//   fn pos( &mut self ) -> &mut d2::V
//   {
//     BoxCollisionPositonResolutionTrait::pos( self )
//   }
//   /// Inverse mass.
//   #[ inline( always ) ]
//   fn inv_mass( &self ) -> d2::S
//   {
//     BoxCollisionPositonResolutionTrait::inv_mass( self )
//   }
// }

/// Handle collision.
#[ inline ]
pub fn collision_position_solve
(

  collision : &'_ d2::Collision,

  // a : &mut impl CollisionPositonResolutionTrait,
  // b : &mut impl CollisionPositonResolutionTrait,

  a_pos : &'_ mut d2::V,
  a_inv_mass : d2::S,
  // a_restitution : d2::S,

  b_pos : &'_ mut d2::V,
  b_inv_mass : d2::S,
  // b_restitution : d2::S,

)
{

  if collision.depth > 0.
  {
    // println!( "collision : {collision:?}" );
    let w = a_inv_mass + b_inv_mass;
    *a_pos -= collision.normal * collision.depth * a_inv_mass / w;
    *b_pos += collision.normal * collision.depth * b_inv_mass / w;
  };

  // if collision.depth > 0.
  // {
  //   // println!( "collision : {collision:?}" );
  //   let w = a.inv_mass() + b.inv_mass();
  //   let a_inv_mass = a.inv_mass();
  //   let b_inv_mass = b.inv_mass();
  //   *a.pos() -= collision.normal * collision.depth * a_inv_mass / w;
  //   *b.pos() += collision.normal * collision.depth * b_inv_mass / w;
  // };

}
