extern crate libc;

use libc::{c_char, c_int};
use std::mem;

pub use ffi::LDAP;

mod ffi;

pub fn ldap_open(host: &str, port: uint) -> Option<LDAP>
{
    let c_host = host.to_c_str();
    unsafe
    {
        let ldap_ptr = ffi::ldap_open(c_host.as_ptr(), port as c_int);
        if ldap_ptr.is_null() { return None; }
        Some(*ldap_ptr)
    }
}

pub fn ldap_init(host: &str, port: uint) -> Option<LDAP>
{
    let c_host = host.to_c_str();
    unsafe
    {
        let ldap_ptr = ffi::ldap_init(c_host.as_ptr(), port as c_int);
        if ldap_ptr.is_null() { return None; }
        Some(*ldap_ptr)
    }
}

pub fn ldap_bind(ld: &LDAP, who: &str, cred: &str, method: int) -> int
{
    let c_who = who.to_c_str();
    let c_cred = cred.to_c_str();
    unsafe
    {
        let res = ffi::ldap_bind(ld, c_who.as_ptr(), c_cred.as_ptr(), method as c_int);
        return res as int;
    }
}
