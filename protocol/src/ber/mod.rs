use std;

pub mod error;

pub mod common;
mod encoder;
mod decoder;
pub mod types;

pub use self::encoder::encode;
pub use self::decoder::decode;

pub type Result<T> = std::result::Result<T, error::ASN1Error>;
