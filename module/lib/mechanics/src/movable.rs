//!
//! Describe movable objects.
//!

use crate::*;
use Game;
// use math::{ d2::V, Cbox };
use math::d2;
use math::d2::prelude::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Movable object.
///

#[ derive( Debug ) ]
pub struct Movable
{

  /// Previous position.
  pub prev_pos : d2::V,
  /// Region current movable occupy. Represented as centered box.
  pub pos : d2::V,
  /// Radius of movable.
  pub radius : d2::S,

  /// 2D acceleration.
  pub acceleration : d2::V,
  /// Velocity of movement.
  pub velocity : d2::V,
  /// Next value of velocity. Necessary for integration.
  pub velocity_before_integration : d2::V,

  /// Velocity is multiplied by `1.0 - Game::Friction * self.friction` on each integration.
  pub friction : d2::S,
  /// Inverted mass of movable.
  pub inv_mass : d2::S,

  // xxx : new types for each component
}

impl Default for Movable
{
  fn default() -> Self
  {
    let pos = d2::V::new( 0.0, 0.0 );
    let radius = Game::PawnRadius;
    let prev_pos = pos;
    let velocity = d2::V::new( 0.0, 0.0 );
    let velocity_before_integration = velocity;
    let acceleration = d2::V::new( 0.0, 0.0 );
    let friction = Game::DefaultPawnFriction;
    let inv_mass = 1.0;
    Self { pos, radius, prev_pos, acceleration, velocity, velocity_before_integration, friction, inv_mass }
  }
}

impl Movable
{

  /// Constuct PlayerComponent Pawn.
  pub fn PawnPlayer() -> Self
  {
    let mut result = Self::default();
    result.friction = Game::DefaultPlayerFriction;
    result
  }

  /// Construct NPC pawn.
  pub fn PawnNpc
  (
    mut playground_box : d2::Cbox,
  ) -> Self
  {
    let mut result = Self::default();

    playground_box.size -= result.radius;
    result.pos = playground_box.random_point();
    result.prev_pos = result.pos;
    result.velocity = d2::V::Random().normalize() * Game::MovableNpcSpeed;
    result.friction = 0.;
    result

//     // let size = d2::V::new( 3.0, 3.0 );
//     let radius = 1.5;
//     playground_box.size -= radius;
//     let pos = playground_box.random_point();
//     // let cbox = Cbox { center, size };
//     // let velocity = d2::V::new( 0.0, 0.0 );
//     // let prev_pos = center + d2::V::Random().normalize() * Game::MovableNpcSpeed;
//     let velocity = d2::V::Random().normalize() * Game::MovableNpcSpeed;
//     let prev_pos = pos;
//
//     // let direction = d2::V::new( 0.0, 1.0 );
//     let acceleration = d2::V::new( 0.0, 0.0 );
//     let inv_mass = 1.0;
//     Self { prev_pos, pos, radius, velocity, acceleration, inv_mass }

  }

  /// Get sphere from position.
  pub fn sphere( &self ) -> d2::Sphere
  {
    d2::Sphere::FromCenterAndRadius( self.pos, self.radius )
  }

  /// Handle 2D move input.
  pub fn movable_player_input
  (
    &mut self,
    optional_direction : Option< d2::V >,
    // xxx : introduce new type
  )
  {

    if let Some( direction ) = optional_direction
    {
      if direction.length() > 0.0
      {
        self.acceleration = direction.normalize() * Game::MovablePlayerAcceleration;
      }
      else
      {
        self.acceleration *= 0.0;
      }
    }
    else
    {
      self.acceleration *= 0.0;
    }

  }

  /// Integrate position based on time and speed. Constraints are in `integrate2`.
  pub fn integrate1
  (
    &mut self,
    delta_seconds : d2::S,
  ) -> bool
  {

    let mut velocity = self.velocity + self.acceleration * delta_seconds * self.inv_mass;
    velocity *= 1.0 - Game::Friction * self.friction;
    self.velocity_before_integration = velocity;

    self.prev_pos = self.pos;
    self.pos = self.pos + velocity * delta_seconds;

    let speed = velocity.length();
    if speed == 0.0
    {
      return false;
    }
    return true
  }

  /// Collide two movables.
  pub fn movable_movable_collide
  (
    &mut self,
    b : &mut Movable,
  ) -> Option< ( d2::Collision, Sound ) >
  {
    let some_collision = self.sphere().sphere_collide_solid( &b.sphere() );
    if let Some( collision ) = some_collision
    {
      physics::collision_position_solve
      (
        &collision,
        &mut self.pos,
        self.inv_mass,
        &mut b.pos,
        b.inv_mass,
      );
      if collision.depth > 0.
      {
        let speed = ( self.velocity - b.velocity ).length_squared();
        if speed > 75.
        {
          // println!( "speed : {speed} | depth : {}", collision.depth );
          return Some( ( collision, Sound::HitPawns ) )
        }
        else
        {
          return Some( ( collision, Sound::None ) )
        }
      };
    }

    None
  }

  /// Constrain movable by boxy playground. Produce hit sound on violation of constrain.
  pub fn playground_collide
  (
    &mut self,
    playground : &Playground,
  ) -> Option< ( d2::Collision, Sound ) >
  {
    let some_collision = self.sphere().cbox_collide_hollow( &playground.cbox );
    if let Some( collision ) = some_collision
    {
      let mut center = playground.cbox.center;
      physics::collision_position_solve
      (
        &collision,
        &mut self.pos,
        self.inv_mass,
        &mut center,
        playground.inv_mass,
      );
      if collision.depth > 0.
      {
        let spead_sqr = self.velocity.length_squared();
        if spead_sqr > 6.0
        {
          // println!( "spead_sqr : {spead_sqr} | depth : {}", collision.depth );
          return Some( ( collision, Sound::HitBorder ) );
        }
        else
        {
          return Some( ( collision, Sound::None ) );
        }
      };
    }

    return None;
  }

  /// Integrate position and adjust velocity after constraints.
  pub fn integrate2
  (
    &mut self,
    delta_seconds : d2::S,
  ) -> bool
  {

    if delta_seconds > 1e-7
    {
      self.velocity = ( self.pos - self.prev_pos ) / delta_seconds;
    }

    return true
  }

  //

}

/// Collision velosity resolving. Finalt stage.
pub fn collision_velocity_resolve
(
  m1 : &mut Movable,
  m2 : &mut Movable,
)
{

  let normal = ( m2.pos - m1.pos ).normalize();

  if normal.is_nan()
  {
    return;
  }

  let velocity_before_integration = m2.velocity_before_integration - m1.velocity_before_integration;
  let normal_speed_before_integration = d2::V::dot( velocity_before_integration, normal );
  let velocity_after_integration = m2.velocity - m1.velocity;
  let normal_speed_after_integration = d2::V::dot( velocity_after_integration, normal );
  let restitution = 1.;
  let w = m1.inv_mass + m2.inv_mass;

  m1.velocity -= normal * ( -normal_speed_after_integration - restitution*normal_speed_before_integration ) * m1.inv_mass / w;
  m2.velocity += normal * ( -normal_speed_after_integration - restitution*normal_speed_before_integration ) * m2.inv_mass / w;

  // println!( "normal : {normal:?}" );
  // println!( "normal_speed_after_integration : {normal_speed_after_integration:?}" );
  // println!( "normal_speed_before_integration : {normal_speed_before_integration:?}" );
  // println!( "w : {w:?}" );
  // println!( "m1.velocity : {:?}", m1.velocity );
  // println!( "m2.velocity : {:?}", m2.velocity );

}
