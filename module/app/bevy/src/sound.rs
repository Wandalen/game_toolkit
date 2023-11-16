//!
//! Sound management.
//!

use crate::*;

use kit::rand::prelude::*;
use kit::generic::map_of_vectors::{ MapOfVectors, MapOfVectorsFormer, FromFormerOfMapOfVectors };

pub use mechanics::Sound;

/// Namespace to include with asterisk.
pub mod prelude
{
  use crate::*;
  pub use bevy::prelude::*;
}

/// Collection of sounds.
///
/// TODO: Try using bevy::asset::Assets.
#[ derive( Resource, Default, Debug ) ]
pub struct SoundsResource( MapOfVectors< Sound, Handle< AudioSource > > );
/// Alias of former ( aka builder ) of map of vectors of sounds.
pub type SoundsResourceFormer = MapOfVectorsFormer< SoundsResource, Sound, String, Handle< AudioSource > >;

impl SoundsResource
{
  /// Start process of mutating of the SoundsResource creating former ( aka builder ).
  #[ inline( always ) ]
  pub fn reform( self, asset_server : Res< '_, AssetServer > ) -> SoundsResourceFormer
  {
    let asset_server = asset_server.clone();
    let maker = move | _k : &_, file_path : String | asset_server.load::< AudioSource, &str >( &file_path );
    self.0
    .reform::< SoundsResource, String >( Box::new( maker ) )
  }
  /// Start process of mutating a new SoundsResource creating former ( aka builder ).
  #[ inline( always ) ]
  pub fn Preform( asset_server : Res< '_, AssetServer > ) -> SoundsResourceFormer
  {
    Self::default().reform( asset_server )
  }
}

impl FromFormerOfMapOfVectors< Sound, String, Handle< AudioSource > >
for SoundsResource
{
  fn from_former( former : SoundsResourceFormer ) -> Self
  {
    SoundsResource( former.dst )
  }
}

/// Request to play sound.
#[ derive( Event, Debug ) ]
pub struct SoundEvent
{
  /// Sound to play.
  pub sound : Sound,
}

/// Setup sound system.
pub fn setup_fn
(
  mut commands : Commands< '_, '_ >,
  asset_server : Res< '_, AssetServer >,
)
{

  let sounds = SoundsResource::Preform( asset_server )
  // .add( Sound::HitBorder, "audio/pluck_001.ogg" )
  // .add( Sound::HitBorder, "audio/pluck_002.ogg" )
  .add( Sound::HitBorder, "audio/metal_plate_c.wav" )
  .add( Sound::HitBorder, "audio/metal_plate_d.wav" )
  .add( Sound::HitBorder, "audio/metal_plate_e.wav" )
  .add( Sound::HitPawns, "audio/chainmail_hit_b.wav" )
  .add( Sound::HitPawns, "audio/chainmail_hit_c.wav" )
  .add( Sound::HitPawns, "audio/chainmail_hit_d.wav" )
  .add( Sound::HitPawns, "audio/chainmail_hit_f.wav" )

  .form()
  ;



  commands.insert_resource
  (
    sounds
  );

}

/// Play sound on event to play sound.
pub fn play_fn
(
  mut commands : Commands< '_, '_ >,
  mut event_reader : EventReader< '_, '_, SoundEvent >,
  sounds_resource : Res< '_, SoundsResource >,
)
{

  // return;

  let settings = PlaybackSettings
  {
    paused : false,
    mode : bevy::audio::PlaybackMode::Once,
    ..default()
  };

  for event in event_reader.iter()
  {

    match event.sound
    {
      Sound::HitBorder | Sound::HitPawns =>
      {
        if let Some( sounds ) = sounds_resource.0.get( event.sound )
        {

          // println!( "sounds.len() : {}", sounds.len() );
          if sounds.len() < 1
          {
            continue;
          }

          let sound = &sounds[ ( random::< f32 >() * sounds.len() as f32 - 0.001 ).floor() as usize ];

          commands.spawn
          ((
            AudioBundle
            {
              source : sound.clone(),
              settings : settings.clone(),
              ..default()
            },
          ));

        }
      }
      Sound::None =>
      {
      }
      #[ allow( unreachable_patterns ) ]
      _ =>
      {
        warn!( "Uncaught sound event {:?}", &event );
      }
    }
  }

}
