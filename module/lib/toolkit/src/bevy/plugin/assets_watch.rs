//!
//! Track all assets events.
//!

use bevy::prelude::*;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// Track all assets events.
///
#[ derive( Debug ) ]
pub struct AssetsWatchPlugin;
/// Alias for the plugin defined here.
pub type Plugin = AssetsWatchPlugin;
impl bevy::app::Plugin for AssetsWatchPlugin
{
  fn build( &self, app : &mut App )
  {
    app
    .add_systems( Update, assets_events_fn )
    ;
  }
}

//

use bevy::gltf::Gltf;
use bevy::asset::{ Asset, HandleId  };

/// Extract handle from `AssetEvent`.
pub fn handle_debug< T >
(
  asset_server : Res< '_, AssetServer >,
  src : Handle< T >
) -> String
where
  T : Asset,
{
  match src.id()
  {
    id @ HandleId::Id( _, _ ) => format!( "{id:?}" ),
    HandleId::AssetPathId( path_id ) => format!( "{:?}", asset_server.get_handle_path( path_id ) ),
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

/// Handle events.
pub fn assets_events_fn
(
  asset_server : Res< '_, AssetServer >,
  // images : Res< '_, Assets< Image > >,
  mut image_events : EventReader< '_, '_, AssetEvent< Image > >,
  mut audio_events : EventReader< '_, '_, AssetEvent< AudioSource > >,
  mut mesh_events : EventReader< '_, '_, AssetEvent< Mesh > >,
  mut shader_events : EventReader< '_, '_, AssetEvent< Shader > >,
  mut gltf_events : EventReader< '_, '_, AssetEvent< Gltf > >,
  mut font_events : EventReader< '_, '_, AssetEvent< Font > >,
  mut scene_events : EventReader< '_, '_, AssetEvent< Scene > >,
)
{

  for event in image_events.iter()
  {
    info!( "Image loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in audio_events.iter()
  {
    info!( "Audio loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in mesh_events.iter()
  {
    info!( "Mesh loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in shader_events.iter()
  {
    info!( "Shader loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in gltf_events.iter()
  {
    info!( "GLTF loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in font_events.iter()
  {
    info!( "Fonts loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

  for event in scene_events.iter()
  {
    info!( "Scene loaded : {:?}", handle_debug( Res::clone( &asset_server ), asset_event_handle( event ) ) );
  }

}
