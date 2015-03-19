use std::ptr;
use std::str;
use std::mem;
use std::boxed;
use std::ffi::{CString, CStr};

use libc;

use ffi;
use message::LDAPMessage;

/// This struct represents a connection to a ldap server
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

    pub fn unbind(&mut self) -> bool
    {
        let res: i32;
        unsafe
        {
            res = ffi::ldap_unbind_s(self.ptr) as i32;
        }

        if res == 0 { return true; }

        ldap_err2string(res);

        false
    }

    pub fn set_option(&mut self) -> bool
    {
        unsafe
        {

            let option: Box<libc::c_void> = mem::transmute(Box::new(ffi::LDAP_VERSION3));
            ffi::ldap_set_option(self.ptr, ffi::LDAP_OPT_PROTOCOL_VERSION as libc::c_int, boxed::into_raw(option));
        }
        false
    }

    pub fn search(&mut self, base: &str, scope: isize, filter: &str, attrs: &str, limit: i32) -> Option<LDAPMessage>
    {
        let c_base = CString::new(base).unwrap();
        let c_filter = CString::new(filter).unwrap();
        let c_attrs = Box::new(CString::new(attrs).unwrap().as_ptr());

        let mut msg: *mut ffi::LDAPMessage = ptr::null_mut();
        let res: i32;
        unsafe
        {
            res = ffi::ldap_search_s(self.ptr, c_base.as_ptr(), scope as libc::c_int, c_filter.as_ptr(), boxed::into_raw(c_attrs), 0, &mut msg) as i32;
        }

        if res == 0
        {
            let mut lmsg = LDAPMessage::from_ptr(msg);
            return Some(lmsg);
        }

        println!("{}", ldap_err2string(res));

        None
    }

    pub unsafe fn get_ptr(&mut self) -> *mut ffi::LDAP
    {
        self.ptr
    }

}

pub fn ldap_err2string(err: i32) -> String
{
    unsafe
    {
        let c_res = ffi::ldap_err2string(err as ::libc::c_int);
        let res = CStr::from_ptr(c_res);
        return str::from_utf8(res.to_bytes()).unwrap().to_string();
    }
}


