
use std::fmt;

use libc::{c_char, c_int, c_void};

pub static LDAP_VERSION1: isize = 1;
pub static LDAP_VERSION2: isize = 2;
pub static LDAP_VERSION3: isize = 3;

pub static LDAP_OPT_PROTOCOL_VERSION: isize = 0x0011;

pub static LDAP_SUCCESS: i32 = 0x00;

#[link(name = "ldap")]
extern
{
    pub fn ldap_initialize(ld: *mut *mut LDAP, url: *const c_char) -> c_int;
    pub fn ldap_simple_bind_s(ld: *mut LDAP, who: *const c_char, passwd: *const c_char) -> c_int;
    pub fn ldap_err2string(err: c_int) -> *mut c_char;
    pub fn ldap_set_option(ld: *mut LDAP, option: c_int, invalue: *const c_void) -> c_int;
}

pub enum Struct_ldap { }
pub type LDAP = Struct_ldap;
