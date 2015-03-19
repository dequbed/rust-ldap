use std::ptr;
use std::str;
use std::ffi::CStr;

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

    pub fn is_null(&self) -> bool
    {
        self.ptr.is_null()
    }

    pub unsafe fn get_ptr(&mut self) -> *mut ffi::LDAPMessage
    {
        self.ptr
    }
}
