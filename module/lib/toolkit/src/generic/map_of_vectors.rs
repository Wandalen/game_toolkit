//!
//! Implements map of vectors with former pattern to form and reform it.
//! Optionally accept function make to produce value out of raw data.
//!

#![ allow( dead_code ) ]
#![ allow( non_snake_case ) ]

use core::marker::PhantomData;
use std::hash::Hash;
use std::collections::HashMap;
use std::fmt;

/// Namespace to include with asterisk.
pub mod prelude
{
}

/// Restrictions for callback intended to construct a value.

pub trait MakerTrait< Key, PreVal, Val >
where
  Self : FnMut( &Key, PreVal ) -> Val,
  Val : ValTrait,
  PreVal : Clone,
{}

impl< Key, PreVal, Val, T > MakerTrait< Key, PreVal, Val > for T
where
  T : FnMut( &Key, PreVal ) -> Val,
  Self : FnMut( &Key, PreVal ) -> Val,
  Val : ValTrait,
  PreVal : Clone,
{
}

/// Dynamic maker.
pub type MakerDyn< Key, PreVal, Val > = Box< dyn MakerTrait< Key, PreVal, Val > >;

/// Restriction for key of map.

pub trait KeyTrait
where
  Self : fmt::Debug + Default + Eq + PartialEq + Hash,
{}

impl< T > KeyTrait for T
where
  Self : fmt::Debug + Default + Eq + PartialEq + Hash,
  T : fmt::Debug + Default + Eq + PartialEq + Hash,
{}

/// Restritctions for value of map.

pub trait ValTrait
where
  Self : fmt::Debug + Clone + Default,
{}

impl< T > ValTrait for T
where
  Self : fmt::Debug + Clone + Default,
  T : fmt::Debug + Clone + Default,
{}

///
/// Map of vectors with former pattern to form and reform it.
/// Optionally accept function make to produce value out of raw data.
///

#[ derive( Debug, Default ) ]
pub struct MapOfVectors< Key, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
{
  /// Has map of vectors.
  pub map : HashMap< Key, Vec< Val > >,
}

impl< Key, Val >
MapOfVectors< Key, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
{

  /// Default constructor.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Default::default()
  }

  /// Start mutating of an the target object returning Former ( aka Builder ).
  #[ inline( always ) ]
  pub fn reform< FromFormer, PreVal >( self, maker : MakerDyn< Key, PreVal, Val > )
  -> MapOfVectorsFormer< FromFormer, Key, PreVal, Val >
  where
    PreVal : Clone,
    FromFormer : FromFormerOfMapOfVectors< Key, PreVal, Val >,
  {
    let _phantom = PhantomData;
    MapOfVectorsFormer { dst : self, maker, _phantom }
  }

  /// Start creating of an the target object returning a Former ( aka Builder ) wrapping a new instance of target object.
  #[ inline( always ) ]
  pub fn Preform< FromFormer, PreVal >( maker : MakerDyn< Key, PreVal, Val > )
  -> MapOfVectorsFormer< FromFormer, Key, PreVal, Val >
  where
    PreVal : Clone,
    FromFormer : FromFormerOfMapOfVectors< Key, PreVal, Val >,
  {
    Self::default().reform( maker )
  }

  /// Start creating of an the target object returning a Former ( aka Builder ) wrapping a new instance of target object.
  /// Imply that there is no make to converting values. Assuming values are used as is.
  #[ inline( always ) ]
  pub fn PreformDefault()
  -> MapOfVectorsFormer< Self, Key, Val, Val >
  {
    Self::default().reform( Box::new(| _k, v | v ) )
  }

  /// Get vector of values with the key.
  #[ inline( always ) ]
  pub fn get< IntoKey >( &self, k : IntoKey ) -> Option< &Vec< Val > >
  where
    IntoKey : Into< Key >
  {
    self.map.get( &k.into() )
  }

}

///
/// Former of map of vectors.
///
/// Could be obtained from map of vectors by function `reform` and be converted into map of vectors by function `form`.
///

pub struct MapOfVectorsFormer< FromFormer, Key, PreVal, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
  PreVal : Clone,
  FromFormer : FromFormerOfMapOfVectors< Key, PreVal, Val >,
{
  /// Destination map of vectors to mutate with the former.
  pub dst : MapOfVectors< Key, Val >,
  /// Preprocess each element converting `PreVal` to `Val`.
  /// If no converstion is needed then just use identity closure.
  pub maker : MakerDyn< Key, PreVal, Val >,
  _phantom : PhantomData< dyn Fn() -> ( PreVal, FromFormer ) >,
}

impl< FromFormer, Key, PreVal, Val > fmt::Debug
for MapOfVectorsFormer< FromFormer, Key, PreVal, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
  PreVal : Clone,
  FromFormer : FromFormerOfMapOfVectors< Key, PreVal, Val >,
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    f
    .debug_struct("MapOfVectorsFormer" )
    .field( "dst", &self.dst )
    .finish()
  }
}

impl< FromFormer, Key, PreVal, Val >
MapOfVectorsFormer< FromFormer, Key, PreVal, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
  PreVal : Clone,
  FromFormer : FromFormerOfMapOfVectors< Key, PreVal, Val >,
{
  /// Add key-value pair to the map of vectors. Create a new vector if it didn't exist.
  #[ inline( always ) ]
  pub fn add< IntoKey, IntoPreVal >( mut self, key : IntoKey, val : IntoPreVal ) -> Self
  where
    IntoKey : Into< Key >,
    IntoPreVal : Into< PreVal >,
  {
    let key = key.into();
    let val = ( self.maker )( &key, val.into() );
    self.dst.map.entry( key )
    .or_insert_with( || vec![] )
    .push( val )
    ;
    self
  }
  /// Finish forming and return formed instance.
  #[ inline( always ) ]
  pub fn form( self ) -> FromFormer
  {
    FromFormer::from_former( self )
  }
}

///
/// Produce itself from former of map. It's necessary to produce instance of a custom type as result of forming.
///
/// Called by function `form` of former.
///

pub trait FromFormerOfMapOfVectors< Key, PreVal, Val >
where
  Key : KeyTrait,
  PreVal : Clone,
  Val : ValTrait,
{
  /// Convert from former ( aka builde ) into target ( final ) structure.
  fn from_former( former : MapOfVectorsFormer< Self, Key, PreVal, Val > ) -> Self
  where
    Self : Sized,
  ;
}

impl< Key, PreVal, Val > FromFormerOfMapOfVectors< Key, PreVal, Val >
for MapOfVectors< Key, Val >
where
  Key : KeyTrait,
  Val : ValTrait,
  PreVal : Clone,
{
  fn from_former( former : MapOfVectorsFormer< Self, Key, PreVal, Val > ) -> Self
  where
    Key : KeyTrait,
    Val : ValTrait,
    Self : Sized,
  {
    former.dst
  }
}

//
// = Tests
//

#[ test ]
fn basic_test()
{

  let mut got : MapOfVectors::< &'static str, String > = MapOfVectors::default();
  got = got
  .reform::< MapOfVectors< &'static str, String >, String >( Box::new( | _k, v | v.clone() + &v ) )
  .add( "k1",  "def" )
  .add( "k2", "xyz" )
  .add( "k2", "b" )
  .add( "k2", "c" )
  .form()
  ;
  let mut exp = HashMap::new();
  exp.insert( "k1", vec![ "defdef".to_string() ] );
  exp.insert( "k2", vec![ "xyzxyz".to_string(), "bb".to_string(), "cc".to_string() ] );
  assert_eq!( got.map, exp );

}

#[ test ]
fn custom_target_object_test()
{

  struct Wrap( MapOfVectors< String, String > );

  impl< PreVal >
  FromFormerOfMapOfVectors< String, PreVal, String >
  for Wrap
  where
    PreVal : Clone,
  {
    fn from_former( former : MapOfVectorsFormer< Self, String, PreVal, String > ) -> Self
    where
      Self : Sized,
    {
      Wrap( former.dst )
    }
  }

  let got = MapOfVectors::default()
  .reform::< Wrap, String >( Box::new( | _k, v | v.clone() + &v ) )
  .add( "k1",  "def" )
  .add( "k2", "xyz" )
  .add( "k2", "b" )
  .add( "k2", "c" )
  .form()
  ;
  let mut exp = HashMap::new();
  exp.insert( "k1".to_string(), vec![ "defdef".to_string() ] );
  exp.insert( "k2".to_string(), vec![ "xyzxyz".to_string(), "bb".to_string(), "cc".to_string() ] );
  assert_eq!( got.0.map, exp );

}

#[ test ]
fn preform_test()
{

  let got = MapOfVectors::Preform::< MapOfVectors< String, String >, String >
  ( Box::new( | _k, v | v.clone() + &v ) )
  .add( "k1",  "def" )
  .add( "k2", "xyz" )
  .add( "k2", "b" )
  .add( "k2", "c" )
  .form()
  ;
  let mut exp = HashMap::new();
  exp.insert( "k1".to_string(), vec![ "defdef".to_string() ] );
  exp.insert( "k2".to_string(), vec![ "xyzxyz".to_string(), "bb".to_string(), "cc".to_string() ] );
  assert_eq!( got.map, exp );

}

#[ test ]
fn preform_default_test()
{

  let got = MapOfVectors::PreformDefault()
  .add( "k1",  "def" )
  .add( "k2", "xyz" )
  .add( "k2", "b" )
  .add( "k2", "c" )
  .form()
  ;
  let mut exp = HashMap::new();
  exp.insert( "k1".to_string(), vec![ "def".to_string() ] );
  exp.insert( "k2".to_string(), vec![ "xyz".to_string(), "b".to_string(), "c".to_string() ] );
  assert_eq!( got.map, exp );

}

//
// = Main
//

fn main(){}
