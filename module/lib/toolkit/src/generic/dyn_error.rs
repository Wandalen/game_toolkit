//!
//! Helpers to handle errors.
//!

/// Namespace to include with asterisk.
pub mod prelude
{
}

/// Default dynamic error. Based on `std::error::Error`.
pub type DynError = Box< dyn std::error::Error >;
/// Result with dynamic error based on `std::error::Error`.
pub type DynResult< T > = Result< T, Box< dyn std::error::Error > >;

/// Short-cut to construct `Result< T, DynError >` with defined description.
pub fn err_make< T >( description : &'_ str ) -> Result< T, DynError >
{
  Err( Box::new( std::io::Error::new( std::io::ErrorKind::Other, description ) ) )
}
