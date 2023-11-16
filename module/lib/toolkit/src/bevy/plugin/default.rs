
//!
//! An opinionated alternative set of default plugins with improved defaults, serving as an alternative to the official default set of plugins.
//!

use crate::bevy;
use bevy::prelude::*;
use std::path::PathBuf;

/// Namespace to include with asterisk.
pub mod prelude
{
}

///
/// An opinionated alternative set of default plugins with improved defaults, serving as an alternative to the official default set of plugins.
///
#[ derive( Debug, Clone ) ]
pub struct DefaultPlugin
{
  /// Path to workspace. If not specified then current path is used.
  pub workspace_path : PathBuf,
}
/// Alias for the plugin defined here.
pub type Plugin = DefaultPlugin;
impl Default for Plugin
{
  fn default() -> Self
  {
    let workspace_path = std::env::current_dir().unwrap();
    Self { workspace_path }
  }
}

impl bevy::app::Plugin for Plugin
{
  fn build( &self, app : &mut App )
  {
    #[ cfg( debug_assertions ) ]
    #[ cfg( feature = "bevy_asset" ) ]
    let watch_for_changes = Some( bevy::asset::ChangeWatcher{ delay : bevy::utils::Duration::new( 1, 0 ) } );
    #[ cfg( not( debug_assertions ) ) ]
    let watch_for_changes = None;

    let app = app
    // .add_plugins( DefaultPlugins )

    .add_plugins( bevy::log::LogPlugin::default() )
    .add_plugins( bevy::core::TaskPoolPlugin::default() )
    .add_plugins( bevy::core::TypeRegistrationPlugin::default() )
    .add_plugins( bevy::core::FrameCountPlugin::default() )
    .add_plugins( bevy::time::TimePlugin::default() )
    .add_plugins( bevy::transform::TransformPlugin::default() )
    .add_plugins( bevy::hierarchy::HierarchyPlugin::default() )
    .add_plugins( bevy::diagnostic::DiagnosticsPlugin::default() )
    .add_plugins( bevy::input::InputPlugin::default() )
    .add_plugins( bevy::window::WindowPlugin::default() )
    .add_plugins( bevy::a11y::AccessibilityPlugin )
    ;

    #[ cfg( feature = "bevy_asset" ) ]
    {
      let mut asset_folder = self.workspace_path.clone();
      asset_folder.push::< String >( "asset".into() );
      // dbg!( &asset_folder );
      let asset_folder = asset_folder.into_os_string().into_string().expect("Failed to convert to String");
      app.add_plugins( bevy::asset::AssetPlugin { asset_folder, watch_for_changes } );
    }

    #[ cfg( feature = "bevy_scene" ) ]
    let app = app.add_plugins( bevy::scene::ScenePlugin::default() );

    #[ cfg( feature = "bevy_winit" ) ]
    let app = app.add_plugins( bevy::winit::WinitPlugin::default() );

    #[ cfg( feature = "bevy_render" ) ]
    let app = app.add_plugins( bevy::render::RenderPlugin::default() );
    #[ cfg( feature = "bevy_render" ) ]
    let app = app.add_plugins( bevy::render::texture::ImagePlugin::default() );

    #[ cfg( feature = "bevy_render" ) ]
    #[ cfg( all( not( target_arch = "wasm32" ), feature = "multi-threaded" ) ) ]
    let app = app.add_plugins( bevy::render::pipelined_rendering::PipelinedRenderingPlugin::default() );

    #[ cfg( feature = "bevy_core_pipeline" ) ]
    let app = app.add_plugins( bevy::core_pipeline::CorePipelinePlugin::default() );

    #[ cfg( feature = "bevy_sprite" ) ]
    let app = app.add_plugins( bevy::sprite::SpritePlugin::default() );

    #[ cfg( feature = "bevy_text" ) ]
    let app = app.add_plugins( bevy::text::TextPlugin::default() );

    #[ cfg( feature = "bevy_ui" ) ]
    let app = app.add_plugins( bevy::ui::UiPlugin::default() );

    #[ cfg( feature = "bevy_pbr" ) ]
    let app = app.add_plugins( bevy::pbr::PbrPlugin::default() );

    #[ cfg( feature = "bevy_gltf" ) ]
    let app = app.add_plugins( bevy::gltf::GltfPlugin::default() );

    #[ cfg( feature = "bevy_audio" ) ]
    let app = app.add_plugins( bevy::audio::AudioPlugin::default() );

    #[ cfg( feature = "bevy_gilrs" ) ]
    let app = app.add_plugins( bevy::gilrs::GilrsPlugin::default() );

    #[ cfg( feature = "bevy_animation" ) ]
    let app = app.add_plugins( bevy::animation::AnimationPlugin::default() );

    #[ cfg( feature = "bevy_gizmos" ) ]
    let app = app.add_plugins( bevy::gizmos::GizmoPlugin );

    let _ = app;
  }
}
