#![allow(dead_code, unstable_features, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![feature(libc)]
extern crate libc;

mod ffi;

pub use ldap::LDAP;
pub use message::LDAPMessage;

mod ldap;
mod message;

