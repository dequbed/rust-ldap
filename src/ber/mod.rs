use std;

mod error;

mod common;
mod encoder;

pub type Result<T> = std::result::Result<T, error::ASN1Error>;
