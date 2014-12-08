extern crate libc;

use libc::{c_char, c_int};
use std::mem;

pub use ffi::LDAP;

mod ffi;

pub fn ldap_open(host: &str, port: uint) -> Option<Box<LDAP>>
{
    let c_host = host.to_c_str();
    unsafe
    {
        let ldap_ptr = ffi::ldap_open(c_host.as_ptr(), port as c_int);
        if ldap_ptr.is_null() { return None; }
        let boxed_ptr: Box<LDAP> = mem::transmute(ldap_ptr);
        Some(boxed_ptr)
    }
}

pub fn ldap_init(host: &str, port: uint) -> Option<Box<LDAP>>
{
    let c_host = host.to_c_str();
    unsafe
    {
        let ldap_ptr = ffi::ldap_init(c_host.as_ptr(), port as c_int);
        if ldap_ptr.is_null() { return None; }
        let boxed_ptr: Box<LDAP> = mem::transmute(ldap_ptr);
        Some(boxed_ptr)
    }
}

pub fn ldap_bind(ldbox: Box<LDAP>, who: &str, cred: &str, method: int) -> int
{
    let c_who = who.to_c_str();
    let c_cred = cred.to_c_str();
    unsafe
    {
        let ld: *const LDAP = mem::transmute(ldbox);
        let res = ffi::ldap_bind(ld, c_who.as_ptr(), c_cred.as_ptr(), method as c_int);
        return res as int;
    }
}

pub fn ldap_simple_bind(ldbox: Box<LDAP>, dn: &str, password: &str) -> int
{
    let c_dn = dn.to_c_str();
    let c_password = password.to_c_str();
    unsafe
    {
        let ld: *const LDAP = mem::transmute(ldbox);
        let res = ffi::ldap_simple_bind(ld, c_dn.as_ptr(), c_password.as_ptr());
        return res as int;
    }
}

pub fn ldap_simple_bind_s(ldbox: Box<LDAP>, dn: &str, password: &str) -> int
{
    let c_dn = dn.to_c_str();
    let c_password = password.to_c_str();
    unsafe
    {
        let ld: *const LDAP = mem::transmute(ldbox);
        let res = ffi::ldap_simple_bind_s(ld, c_dn.as_ptr(), c_password.as_ptr());
        return res as int;
    }
}

// Take ownership of the LDAP session handle and invalidate it
pub fn ldap_unbind(ldbox: Box<LDAP>) -> int
{
    unsafe
    {
        let ld: *const LDAP = mem::transmute(ldbox);
        let res = ffi::ldap_unbind(ld);
        return res as int;
    }
}

pub fn ldap_search(ldbox: &Box<LDAP>, base: &str, scope: int, filter: &str, attrs: &[&str], attrsonly: int) -> int
{
    let c_base = base.to_c_str();
    let c_filter = filter.to_c_str();
    let c_attrs = attrs.iter().map(|x| x.to_c_str()).map(|x| x.as_ptr()).collect::<Vec<*const i8>>();
    unsafe
    {
        let ld: *const LDAP = mem::transmute(ldbox);
        let res = ffi::ldap_search(ld, c_base.as_ptr(), scope as c_int, c_filter.as_ptr(), c_attrs.as_slice(), attrsonly as c_int);
        return res as int;
    }
}
