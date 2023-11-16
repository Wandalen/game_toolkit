//!
//! Bevy-specific helpers for game development.
//!
pub mod plugin;
#[ cfg( feature = "bevy" ) ]
pub use bevy::*;

/// Namespace to include with asterisk.
pub mod prelude
{
  #[ cfg( feature = "bevy" ) ]
  pub use bevy::prelude::*;
  pub use super::plugin::prelude::*;

  // Requred to make derives working.

  pub use bevy::a11y as bevy_a11y;
  pub use bevy::app as bevy_app;
  pub use bevy::core as bevy_core;
  pub use bevy::diagnostic as bevy_diagnostic;
  pub use bevy::ecs as bevy_ecs;
  pub use bevy::input as bevy_input;
  pub use bevy::log as bevy_log;
  pub use bevy::math as bevy_math;
  pub use bevy::ptr as bevy_ptr;
  pub use bevy::reflect as bevy_reflect;
  pub use bevy::tasks as bevy_tasks;
  pub use bevy::time as bevy_time;
  pub use bevy::hierarchy as bevy_hierarchy;
  pub use bevy::transform as bevy_transform;
  pub use bevy::utils as bevy_utils;
  pub use bevy::window as bevy_window;

  #[ cfg( feature = "bevy_asset" ) ]
  pub use bevy::asset as bevy_asset;
  #[ cfg( feature = "bevy_scene" ) ]
  pub use bevy::scene as bevy_scene;
  #[ cfg( feature = "bevy_animation" ) ]
  pub use bevy::animation as bevy_animation;
  #[ cfg( feature = "bevy_audio" ) ]
  pub use bevy::audio as bevy_audio;
  #[ cfg( feature = "bevy_core_pipeline" ) ]
  pub use bevy::core_pipeline as bevy_core_pipeline;
  #[ cfg( feature = "bevy_gilrs" ) ]
  pub use bevy::gilrs as bevy_gilrs;
  #[ cfg( feature = "bevy_gltf" ) ]
  pub use bevy::gltf as bevy_gltf;
  #[ cfg( feature = "bevy_pbr" ) ]
  pub use bevy::pbr as bevy_pbr;
  #[ cfg( feature = "bevy_render" ) ]
  pub use bevy::render as bevy_render;
  #[ cfg( feature = "bevy_sprite" ) ]
  pub use bevy::sprite as bevy_sprite;
  #[ cfg( feature = "bevy_text" ) ]
  pub use bevy::text as bevy_text;
  #[ cfg( feature = "bevy_ui" ) ]
  pub use bevy::ui as bevy_ui;
  #[ cfg( feature = "bevy_winit" ) ]
  pub use bevy::winit as bevy_winit;
  #[ cfg( feature = "bevy_gizmos" ) ]
  pub use bevy::gizmos as bevy_gizmos;
  #[ cfg( feature = "bevy_dynamic_plugin" ) ]
  pub use bevy::dynamic_plugin as bevy_dynamic_plugin;

}
