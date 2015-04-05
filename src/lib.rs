#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern crate libc;

mod ffi;

pub use ldap::LDAP;
pub use message::LDAPMessage;

mod ldap;
mod message;

