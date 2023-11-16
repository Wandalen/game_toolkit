//!
//! To track assets loading process and handle errors.
//!

use bevy::prelude::*;
const DefaultImagePath : &str = "./img/pattern/bw/pattern_48.png";

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use super::AssetServerExt;
}

///
/// To track assets loading process and handle errors.
///

# [derive( Debug ) ]
pub struct AssetLoadingPlugin
{
  default_image_path : String,
}
/// Alias for the plugin defined here.
pub type Plugin = AssetLoadingPlugin;
impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    app
    .insert_resource( AssetLoadingResource { default_image_path : self.default_image_path.clone(), ..default() } )
    .add_event::< AssetImageEvent >()
    .add_systems( Update, AssetLoadingResource::assets_loading_start_fn )
    .add_systems( Update, AssetLoadingResource::assets_loading_handler_fn )
    ;
  }
}

impl Default for AssetLoadingPlugin
{
  fn default() -> Self
  {
    let default_image_path = DefaultImagePath.into();
    Self { default_image_path }
  }
}

///
/// To track assets loading process and handle errors.
///

# [derive( Resource, Debug ) ]
pub struct AssetLoadingResource
{
  /// Path to the image to use as a fallback in case any images fail to load.
  pub default_image_path : String,
  /// Keeps track of a list of images that are currently being loaded.
  /// Once the loading process is complete, the image is removed from the list.
  pub images : Vec< Handle< Image > >,
}

impl AssetLoadingResource
{

  /// Constructor accepting only path to default image to load if no image is found.
  pub fn new( default_image_path : String ) -> Self
  {
    let images = Default::default();
    Self { default_image_path, images }
  }

  /// System to handle loading process of images and replace it with default if error will happen.
  pub fn assets_loading_start_fn
  (
    mut images: EventReader< '_, '_, AssetImageEvent >,
    mut loading : ResMut< '_, AssetLoadingResource >
  )
  {
    for event in images.iter()
    {
      loading.images.push( event.0.clone() );
    }
  }

  /// System to handle loading process of images and replace it with default if error will happen.
  pub fn assets_loading_handler_fn
  (
    mut commands : Commands< '_, '_ >,
    // mut images : Query< '_, '_, Assets< Image > >,
    mut images : Query< '_, '_, ( Entity, &Handle< Image > ) >,
    mut color_materials : Query< '_, '_, ( Entity, &Handle< ColorMaterial >, ) >,
    mut materials: ResMut< '_, Assets< ColorMaterial > >,
    asset_server : Res< '_, AssetServer >,
    mut loading : ResMut< '_, AssetLoadingResource >,
  )
  {
    use bevy::asset::{ LoadState, HandleId };

    // if loading.images.len() > 0
    // {
    //   println!( "images_query : {}", images.iter().count() );
    //   println!( "color_materials_query : {}", color_materials.iter().count() );
    // }

    // asset_server.get_group_load_state()
    let mut to_remove = vec![];
    for( k, handle ) in loading.images.iter().enumerate()
    {
      match handle.id()
      {
        HandleId::AssetPathId( id ) => match asset_server.get_load_state( id )
        {
          LoadState::Loaded =>
          {
            to_remove.push( k );
            // println!( "Loaded {:?}", asset_server.get_handle_path( id ) );
          }
          LoadState::Unloaded =>
          {
            to_remove.push( k );
            println!( "Unloaded {:?}", asset_server.get_handle_path( id ) );
          }
          LoadState::Failed =>
          {
            warn!
            (
              "Failed to load {:?}\nReplacing by {:?}",
              asset_server.get_handle_path( id ),
              &loading.default_image_path,
            );
            let new_image : Handle< Image > = asset_server.load( &loading.default_image_path );
            for( entity, material_id, ) in color_materials.iter_mut()
            {
              let material = materials.get_mut( &material_id ).unwrap();
              let handle2 = material.texture.as_ref().unwrap();
              if handle2.id() == handle.id()
              {
                // warn!( "Replaced {:?}", asset_server.get_handle_path( handle2 ) );
                material.texture = Some( handle2.clone() );
                commands.entity( entity ).insert( materials.add( ColorMaterial::from( new_image.clone() ) ) );
              }
            }
            for ( entity, image, ) in images.iter_mut()
            {
              // println!( "{:?} != {:?}", image.id(), handle.id() );
              if image.id() == handle.id()
              {
                commands.entity( entity ).insert( new_image.clone() );
                // warn!( "Replaced {:?}", asset_server.get_handle_path( id ) );
              }
            }
            // images.set( handle, new_image );
            to_remove.push( k );
          },
          LoadState::Loading | LoadState::NotLoaded =>
          {
            println!( "Loading {:?}", asset_server.get_handle_path( id ) );
          }
          // state @ _ => info!( "{id:?} : {state:?}" ),
        },
        HandleId::Id( _, _ ) => (),
        // _ => (),
      }
    }

    for &e in to_remove.iter().rev()
    {
      loading.images.remove( e );
    }


  }

}

///
/// Event to signal that loading is started.
///
#[ derive( Event, Debug ) ]
pub struct AssetImageEvent( Handle< Image > );

///
/// Extend standard asset server adding extra logic to track progress and handle errors.
///
pub trait AssetServerExt
{
  /// Load image and store it into storage of loading processes to track and handle error if any.
  fn image_load
  (
    &self,
    events : &'_ mut EventWriter< '_, AssetImageEvent >,
    path : &'_ str,
  ) -> Handle< Image >;
}

impl AssetServerExt for AssetServer
{
  fn image_load
  (
    &self,
    events : &'_ mut EventWriter< '_, AssetImageEvent >,
    path : &'_ str,
  ) -> Handle< Image >
  {
    let asset = self.load( path );
    // self.images.push( asset.clone() );
    events.send( AssetImageEvent( asset.clone() ) );
    asset
  }
}

impl Default for AssetLoadingResource
{
  fn default() -> Self
  {
    let default_image_path = DefaultImagePath.into();
    let images = Default::default();
    Self { default_image_path, images }
  }
}

// ///
// /// Extended asset server.
// ///
//
// pub struct AssetServerExt< 'a, 'b, 'c >
// {
//   pub server : Res< 'a, AssetServer >,
//   pub loading : &'b mut ResMut< 'c, AssetLoadingResource >,
// }
//
// // impl< 'a, 'b, 'c > Clone for AssetServerExt< 'a, 'b, 'c >
// // {
// //   fn clone( &self ) -> Self
// //   {
// //     AssetServerExt::new( Res::clone( &self.server ), self.loading )
// //   }
// // }
//
// impl< 'a, 'b, 'c > AssetServerExt< 'a, 'b, 'c >
// {
//   /// Constructor accepting only path to default image to load if no image is found.
//   pub fn new
//   (
//     server : Res< 'a, AssetServer >,
//     loading : &'b mut ResMut< 'c, AssetLoadingResource >,
//   ) -> Self
//   {
//     Self { loading, server }
//   }
//
//   /// Load image string it into storage of loading processes to track and handle error if any.
//   pub fn image_load
//   (
//     &mut self,
//     path : &'_ str,
//   ) -> Handle< Image >
//   {
//     let server = Res::clone( &self.server );
//     self.loading.image_load( server, path )
//   }
//
// }
//
// use std::ops::Deref;
// impl< 'a, 'b, 'c > Deref for AssetServerExt< 'a, 'b, 'c >
// {
//   type Target = Res< 'a, AssetServer >;
//   fn deref( &self ) -> &Self::Target
//   {
//       &self.server
//   }
// }