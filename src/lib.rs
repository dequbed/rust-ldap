#![allow(dead_code, unstable_features, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![feature(libc)]
extern crate libc;

mod ffi;

use std::ptr;
use std::str;
use std::mem;
use std::boxed;
use std::ffi::{CString, CStr};


pub struct LDAP
{
    ptr: *mut ffi::LDAP,
}

impl LDAP
{
    pub fn new() -> LDAP
    {
        LDAP { ptr: ptr::null_mut() }
    }

    pub fn initialize(&mut self, url: &str) -> bool
    {
        let c_url = CString::new(url).unwrap();
        
        let res: i32;
        unsafe
        {
            res = ffi::ldap_initialize(&mut self.ptr, c_url.as_ptr()) as i32;
        }
        
        if res == 0 { return true; }

        false
    }

    pub fn simple_bind(&mut self, who: &str, passwd: &str) -> bool
    {
        let c_who = CString::new(who).unwrap();
        let c_passwd = CString::new(passwd).unwrap();

        let res: i32;
        unsafe
        {
            res = ffi::ldap_simple_bind_s(self.ptr, c_who.as_ptr(), c_passwd.as_ptr()) as i32;
        }

        if res == 0 { return true; }

        ldap_err2string(res);

        false
    }

    pub fn set_option(&mut self) -> bool
    {
        unsafe
        {

            let option: Box<libc::c_void> = mem::transmute(ffi::LDAP_VERSION3);
            ffi::ldap_set_option(self.ptr, ffi::LDAP_OPT_PROTOCOL_VERSION as libc::c_int, boxed::into_raw(option));
        }
        false
    }

    pub fn destroy(self)
    {
        drop(self.ptr);
    }
}

pub fn ldap_err2string(err: i32)
{
    unsafe
    {
        let c_res = ffi::ldap_err2string(err as ::libc::c_int);
        let res = CStr::from_ptr(c_res);
        println!("{}", str::from_utf8(res.to_bytes()).unwrap());
    }
}


