use std;

mod error;

mod common;
mod encoder;
mod decoder;

pub use self::common::*;

pub type Result<T> = std::result::Result<T, error::ASN1Error>;
