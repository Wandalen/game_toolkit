//!
//! Visual.
//!

use crate::*;

// use bevy::prelude::*;
use bevy::asset::Asset;

// use bevy::sprite::MaterialMesh2dBundle;
use bevy::ecs::system::EntityCommands;

use physics::{ MovableComponent, NpcComponent, PlayerComponent, CameraFocusComponent, PlaygroundResource };
use assets::{ AssetImageExt };
use mechanics::{ AssetImage, Pawn };

/// Generic phases for animation.
#[ derive( SystemSet, Debug, Default, Hash, PartialEq, Eq, Clone ) ]
pub enum Phase
{
  /// First phase.
  Pre,
  /// Main phase.
  #[ default ]
  In,
  /// Final phase.
  Post,
}

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
}

/// Playground component.
#[ derive( Component, Default, Debug ) ]
pub struct PlaygroundComponent;

/// Not ready component.
#[ derive( Component, Default, Debug ) ]
pub struct NotReadyComponent;

//

impl MovableComponent
{

  /// Make visual for the pawn.
  pub fn visual_pawn< 'w, 's, 'a1 >
  (
    self,
    commands : &'a1 mut Commands< 'w, 's >,
  ) -> EntityCommands< 'w, 's, 'a1 >
  {

    let transform = Transform::from_translation( d3::V::from( ( self.0.pos.to_screen(), 0.0 ) ) );
    let sprite = Sprite
    {
      custom_size : Some( d2::V::splat( ( &self.0.radius * 2.0 ).to_screen() ) ),
      ..default()
    };

    let ecommands = commands.spawn
    (
      (
        SpriteBundle
        {
          transform : transform.clone(),
          // texture,
          sprite,
          ..default()
        },
        self,
      )
    );

    ecommands
  }

  /// Make visual for NPC pawn.
  pub fn visual_pawn_npc< 'w, 's, 'a >
  (
    self,
    commands : &'a mut Commands< 'w, 's >,
    asset_server : &mut TrackedAssetServer <'_>,
    // mut server : AssetServerExt,
  ) -> EntityCommands< 'w, 's, 'a >
  {
    let texture = AssetImage::Pawn( Pawn::Npc ).load( asset_server );
    let mut r = self.visual_pawn( commands );
    r.insert( ( NpcComponent, texture ) );
    r
  }

  /// Make visual for player pawn.
  pub fn visual_pawn_player< 'w, 's, 'a >
  (
    self,
    commands : &'a mut Commands< 'w, 's >,
    mut asset_server : TrackedAssetServer <'_>,
  ) -> EntityCommands< 'w, 's, 'a >
  {
    let texture = AssetImage::Pawn( Pawn::Player ).load( &mut asset_server );
    let mut r = self.visual_pawn( commands );
    r.insert( ( PlayerComponent, CameraFocusComponent, texture ) );
    r
  }

}

/// Extract handle from `AssetEvent`.
pub fn asset_event_handle< T >( src : &AssetEvent< T > ) -> Handle< T >
where
  T : Asset,
{
  match src
  {
    AssetEvent::Created { handle } => handle.clone(),
    AssetEvent::Modified { handle } => handle.clone(),
    AssetEvent::Removed { handle } => handle.clone(),
  }
}

/// Delayed setup visuals for the world.
pub fn delayed_setup_world_fn
(
  mut commands : Commands< '_, '_ >,
  // mut meshes: ResMut< '_, Assets< Mesh > >,
  // mut materials: ResMut< '_, Assets< ColorMaterial > >,
  mut images: ResMut< '_, Assets< Image > >,
  // playground : Res< '_, PlaygroundResource >,
  images_query : Query< '_, '_, ( Entity, &Handle< Image >, ), ( With< PlaygroundComponent >, With< NotReadyComponent > ) >,
  mut image_events : EventReader< '_, '_, AssetEvent< Image > >,
)
{
  use bevy::render::{ texture::ImageSampler, render_resource::{ AddressMode, FilterMode } };
  use bevy::render::render_resource::SamplerDescriptor;

  if image_events.iter().count() > 0
  {
    for ( entity, image_handle  ) in images_query.iter()
    {
      if let Some( image ) = images.get_mut( &image_handle )
      {
        dbg!( &image.sampler_descriptor );
        let mut descriptor = SamplerDescriptor::default();
        descriptor.address_mode_u = AddressMode::Repeat;
        descriptor.address_mode_v = AddressMode::Repeat;
        descriptor.address_mode_w = AddressMode::Repeat;
        descriptor.min_filter = FilterMode::Linear;
        image.sampler_descriptor = ImageSampler::Descriptor( descriptor );
        commands.entity( entity ).remove::< NotReadyComponent >();
        dbg!( &image.sampler_descriptor );
      }
    }
  }

}

/// Setup visuals for the world.
pub fn setup_world_fn
(
  mut commands : Commands< '_, '_ >,
  // mut meshes: ResMut< '_, Assets< Mesh > >,
  // mut materials: ResMut< '_, Assets< ColorMaterial > >,
  // mut images: ResMut< '_, Assets< Image > >,
  playground : Res< '_, PlaygroundResource >,
  mut asset_server : TrackedAssetServer <'_>,
)
{

  // let color_material = ColorMaterial::from( Color::PURPLE );
  // let color_material = ColorMaterial::from( AssetImage::Background.load( &mut asset_image_events, asset_server ) );
  // let transform = Transform::default().with_scale( d3::V::from( ( to_screen( playground.0.cbox.size ), 1.0 ) ) );

  let texture = AssetImage::Background.load( &mut asset_server );
  // if let Some( texture_body ) = images.get( &texture )
  // {
  //   dbg!( &texture_body.sampler_descriptor );
  // }

  let transform = Transform::from_translation( d3::V::from( ( playground.0.cbox.center, -1.0 ) ) );
  let rect = Some( Rect::from_corners( Vec2::splat( 0.0 ), playground.0.cbox.size * AssetImage::Background.size() ) );
  let sprite = Sprite
  {
    custom_size : Some( playground.0.cbox.size.to_screen() ),
    rect,
    ..default()
  };

  commands.spawn
  (
    (
      SpriteBundle
      {
        transform,
        texture,
        sprite,
        ..default()
      },
      PlaygroundComponent,
      NotReadyComponent,
    )
    // MaterialMesh2dBundle
    // {
    //   mesh : meshes.add( Mesh::from( shape::Quad::default() ) ).into(),
    //   transform : Transform::default().with_scale( d3::V::splat( to_screen( 16.0 ) ) ),
    //   material : materials.add( color_material ),
    //   ..default()
    // }
  );

  // camera

  let transform = Transform::from_xyz( 0.0, 0.0, 0.0 );
  commands.spawn
  (
    Camera2dBundle
    {
      transform,
      ..default()
    }
  );

}

/// Update transformation to reflect position of every movable.
pub fn movable_transformation_update_fn
(
  mut _commands : Commands< '_, '_ >,
  mut query : Query< '_, '_, ( &mut Transform, &MovableComponent ),  >,
)
{
  for( mut transform, movable ) in query.iter_mut()
  {
    transform.translation = d3::V::from( ( movable.0.pos.to_screen(), 0.0 ) );
  }
}

/// Focus every camera on the unique movable with component `CameraFocusComponent`.
pub fn camera_transformation_update_fn
(
  mut _commands : Commands< '_, '_ >,
  focused_movable : Query< '_, '_, ( &MovableComponent, &CameraFocusComponent ),  >,
  mut cameras : Query< '_, '_, ( &mut Transform, &Camera2d ), ( Without< CameraFocusComponent >, ) >,
)
{
  let ( movable, _ ) = focused_movable.single();
  for( mut transform, _camera ) in cameras.iter_mut()
  {
    transform.translation = d3::V::from( ( movable.0.pos.to_screen(), 0.0 ) );
  }
}

/// Ability to be converted from logical space to screen space.
pub trait ToScreen
{

  /// Convert mechanics space to screen space.
  fn to_screen( &self ) -> Self;

}

impl ToScreen for d2::V
{

  /// Convert mechanics space to screen space.
  #[ inline ]
  fn to_screen( &self ) -> Self
  {
    *self * Game::CellSize
  }

}

impl ToScreen for d2::S
{

  /// Convert mechanics space to screen space.
  #[ inline ]
  fn to_screen( &self ) -> Self
  {
    *self * Game::CellSize
  }

}
