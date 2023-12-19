//!
//! Assets management.
//!

use crate::*;
// use bevy::prelude::*;

use kit::sealed;
use mechanics::{ AssetImage, Pawn };
pub use plugin::al::prelude::TrackedAssetServer;

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
  pub use super::plugin::al::prelude::*;
  pub use super::AssetImageExt;
  pub use super::TrackedAssetServer;
}

///
/// Functionality of image assets.
///

#[ sealed ]
pub trait AssetImageExt
{
  /// Size of the image.
  fn size( &self ) -> Vec2;
  /// Load the image asset.
  fn load
  (
    &self,
    asset_server : &mut TrackedAssetServer <'_>,
  ) -> Handle< Image >;
}

#[ sealed ]
impl AssetImageExt for AssetImage
{
  fn size( &self ) -> Vec2
  {
    match self
    {
      Self::Pawn( Pawn::Player ) => Vec2::splat( 64.0 ),
      Self::Pawn( Pawn::Npc ) => Vec2::splat( 64.0 ),
      Self::Background => Vec2::splat( 256.0 ),
      _ =>
      {
        warn!( "Unknwon Sprite Image {:?}", self );
        Vec2::splat( 1.0 )
      },
    }
  }
  fn load
  (
    &self,
    asset_server : &mut TrackedAssetServer <'_>,
  ) -> Handle< Image >
  {
    let path = match self
    {
      Self::Pawn( Pawn::Player ) => "img/ball_blue_large.png",
      Self::Pawn( Pawn::Npc ) => "img/ball_red_large.png",
      Self::Background => "img/pattern/bw/pattern_85.png",
      _ =>
      {
        warn!( "Unknwon Sprite Image {:?}", self );
        "img/pattern/bw/pattern_48.png"
      },
    };
    asset_server.load_tracked( path )
  }
}
