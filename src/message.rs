use std::ptr;
use std::str;
use std::mem;
use std::slice;
use std::ffi::{CStr, CString};

use ffi;
use ldap::LDAP;

/// This struct represents a LDAP Message
pub struct LDAPMessage
{
    ptr: *mut ffi::LDAPMessage,
}

impl LDAPMessage
{
    pub fn new() -> LDAPMessage
    {
        LDAPMessage { ptr: ptr::null_mut() }
    }

    pub fn from_ptr(ptr: *mut ffi::LDAPMessage) -> LDAPMessage
    {
        LDAPMessage { ptr: ptr }
    }

    pub fn count_entries(&mut self, ld: &mut LDAP) -> i32
    {
        unsafe { ffi::ldap_count_entries(ld.get_ptr(), self.ptr) as i32 }
    }

    pub fn first_entry(&mut self, ld: &mut LDAP) -> Option<LDAPMessage>
    {
        unsafe
        {
            let res: *mut ffi::LDAPMessage = ffi::ldap_first_entry(ld.get_ptr(), self.ptr);
            if res.is_null() { return None; }
            Some(LDAPMessage { ptr: res })
        }
    }

    pub fn next_entry(&mut self, ld: &mut LDAP) -> Option<LDAPMessage>
    {
        unsafe
        {
            let res: *mut ffi::LDAPMessage = ffi::ldap_next_entry(ld.get_ptr(), self.ptr);
            if res.is_null() { return None; }
            Some(LDAPMessage { ptr: res })
        }
    }

    pub fn get_dn(&mut self, ld: &mut LDAP) -> String
    {
        unsafe
        {
            let c_res = ffi::ldap_get_dn(ld.get_ptr(), self.ptr);
            let res = CStr::from_ptr(c_res);
            return str::from_utf8(res.to_bytes()).unwrap().to_string();
        }
    }

    pub fn count_values(&mut self) -> i32
    {
        0
    }

    pub fn get_values(&mut self, ld: &mut LDAP, attrs: &str) -> String
    {
        let c_attrs = CString::new(attrs).unwrap();
        let res_slice: &[u8];
        let val: &str;

        unsafe
        {
            let doubleptr = ffi::ldap_get_values_len(ld.get_ptr(), self.ptr, c_attrs.as_ptr());
            res_slice = mem::transmute(slice::from_raw_parts((**doubleptr).bv_val, (**doubleptr).bv_len as usize));
        }

        str::from_utf8(res_slice).unwrap().to_string()
        
    }

    pub fn is_null(&self) -> bool
    {
        self.ptr.is_null()
    }

    pub unsafe fn get_ptr(&mut self) -> *mut ffi::LDAPMessage
    {
        self.ptr
    }
}
