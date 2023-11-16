//! Helper to get value either from reference or from value.
//!
//! # Sample
//!
//!```rust
//!fn f< V >( a : V )
//!where
//!  V : Into< ValueOf< i32 > >,
//!{
//!  let a = a.into().0;
//!  println!( "{a}" );
//!}
//!f( 13 );
//!f( &13 );
//!f( &&13 );
//!```
//!

/// Helper to get value either from reference or from value.
///
/// # Sample
///
///```rust
///fn f< V >( a : V )
///where
///  V : Into< ValueOf< i32 > >,
///{
///  let a = a.into().0;
///  println!( "{a}" );
///}
///f( 13 );
///f( &13 );
///f( &&13 );
///```
///

#[ derive( Debug ) ]
pub struct ValueOf< T >( pub T );

//

impl< T > From< T >
for ValueOf< T >
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    Self( src )
  }
}

//

impl< T > From< &T > for ValueOf< T >
where
  T : Clone,
  Self : From< T >
{
  #[ inline( always ) ]
  fn from( src : &T ) -> Self
  {
    From::from( src.clone() )
  }
}

//

impl< T > From< &&T > for ValueOf< T >
where
  T : Clone,
  Self : From< T >
{
  #[ inline( always ) ]
  fn from( src : &&T ) -> Self
  {
    #[ allow( suspicious_double_ref_op ) ]
    From::from( src.clone() )
  }
}

//

impl< T > From< &&&T > for ValueOf< T >
where
  T : Clone,
  Self : From< T >
{
  #[ inline( always ) ]
  fn from( src : &&&T ) -> Self
  {
    #[ allow( suspicious_double_ref_op ) ]
    From::from( src.clone() )
  }
}
