extern crate byteorder;

pub mod ber;
pub mod bind;
pub mod error;
mod connection;

pub use connection::LDAP;

pub type Result<T> = std::result::Result<T, error::LDAPError>;
