//!
//! Use fyrox game engine to impelemnt visual and other aspectso of mechanics of the game.
//!

#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

/// Namespace to include with asterisk.
pub mod prelude
{
  pub use super::kit::prelude::*;
}
use fyrox::
{
  core::pool::Handle,
  event::Event,
  event_loop::ControlFlow,
  gui::message::UiMessage,
  plugin::{ Plugin, PluginConstructor, PluginContext, PluginRegistrationContext },
  scene::{ Scene, loader::AsyncSceneLoader },
  core::log::Log
};

use fyrox::scene::sprite::SpriteBuilder;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::node::Node;
use fyrox::core::color::Color;
pub use fyrox::engine::executor::Executor;

pub use game_tookit as kit;
use std::fmt;

///
/// Game plugin. The main entity of the crate.
///
#[ derive( Default, Debug ) ]
pub struct GameConstructor;
pub use GameConstructor as Game;

impl PluginConstructor for GameConstructor
{
  fn register( &self, _context: PluginRegistrationContext< '_ > )
  {
    // Register your scripts here.
  }

  fn create_instance
  (
    &self,
    override_scene : Handle< Scene >,
    context : PluginContext< '_, '_ >,
  ) -> Box<dyn Plugin>
  {
    Box::new( GameScene::new( override_scene, context ) )
  }
}

/// Game scene??
#[ derive( Default ) ]
pub struct GameScene
{
  scene : Handle< Scene >,
  loader : Option< AsyncSceneLoader >,
}

impl fmt::Debug for GameScene
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f
    .debug_struct( "GameScene" )
    .field( "scene", &self.scene )
    .finish()
  }
}

impl GameScene
{
  /// Constructor overriding scene by scene loaded from a file.
  pub fn new( override_scene : Handle< Scene >, _context : PluginContext< '_, '_ > ) -> Self
  {
    let loader = None;
    let scene = if override_scene.is_some()
    {
      override_scene
    }
    else
    {
      // loader = Some( AsyncSceneLoader::begin_loading
      // (
      //   "data/scene.rgs".into(),
      //   context.serialization_context.clone(),
      //   context.resource_manager.clone(),
      // ));
      // loader = Some( Default::default() );
      Default::default()
    };
    // Self::create_sprite( &mut scene );
    Self { scene, loader }
  }

  /// Create sprite??
  pub fn create_sprite( scene : &mut Scene ) -> Handle< Node >
  {
    SpriteBuilder::new( BaseBuilder::new() )
    .with_size( 2.0 )
    .with_rotation( 45.0f32.to_radians() )
    .with_color( Color::RED )
    .build( &mut scene.graph )
  }

}

impl Plugin for GameScene
{

  fn on_deinit
  (
    &mut self,
    _context : PluginContext< '_, '_ >,
  )
  {
    // Do a cleanup here.
  }

  fn update
  (
    &mut self,
    context : &mut PluginContext< '_, '_ >,
    _control_flow : &mut ControlFlow,
  )
  {

    if let Some( loader ) = self.loader.as_ref()
    {
      if let Some( result ) = loader.fetch_result()
      {
        match result
        {
          Ok( scene ) =>
          {
            self.scene = context.scenes.add( scene );
          }
          Err( err ) => Log::err( err ),
        }
      }
    }
    println!( "-------- update!" );
    // Add your global update code here.
  }

  fn on_os_event
  (
    &mut self,
    _event : &Event< '_, () >,
    _context : PluginContext< '_, '_ >,
    _control_flow : &mut ControlFlow,
  )
  {
    // Do something on OS event here.
  }

  fn on_ui_message
  (
    &mut self,
    _context : &mut PluginContext< '_, '_ >,
    _message : &UiMessage,
    _control_flow : &mut ControlFlow,
  )
  {
    // Handle UI events here.
  }

}

//

impl kit::Runnable for Game
{
  fn Run() -> kit::DynResult< () >
  {
    let mut executor = Executor::new();
    executor.add_plugin_constructor( GameConstructor );
    executor.run()
  }

}
