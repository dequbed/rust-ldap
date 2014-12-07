use libc::{c_char, c_int};

#[repr(C)]
pub struct LDAP;

#[link(name = "ldap")]
extern
{
    pub fn ldap_open(host: *const c_char, port: c_int) -> *mut LDAP;
    pub fn ldap_init(host: *const c_char, port: c_int) -> *mut LDAP;
    pub fn ldap_initialize(ldp: LDAP, uri: *const c_char) -> c_int;
    pub fn ldap_bind(ld: *const LDAP, who: *const c_char, cred: *const c_char, method: c_int) -> c_int;
}
