use libc::{c_char, c_int};

#[repr(C)]
pub struct LDAP
{
    pub ld_deref: c_int,
    pub ld_timelimit: c_int,
    pub ld_sizelimit: c_int,
    pub ld_errno: c_int,
    pub ld_matched: *mut c_char,
    pub ld_error: *mut c_char,
}

#[link(name = "ldap")]
extern
{
    pub fn ldap_open(host: *const c_char, port: c_int) -> *mut LDAP;
    pub fn ldap_init(host: *const c_char, port: c_int) -> *mut LDAP;
    pub fn ldap_initialize(ld: *const LDAP, uri: *const c_char) -> c_int;
    pub fn ldap_bind(ld: *const LDAP, dn: *const c_char, password: *const c_char, method: c_int) -> c_int;
    pub fn ldap_simple_bind(ld: *const LDAP, dn: *const c_char, password: *const c_char) -> c_int;
    pub fn ldap_unbind(ld: *const LDAP) -> c_int;
    pub fn ldap_search(ld: *const LDAP, base: *const c_char, scope: c_int, filter: *const c_char, attrs: &[*const c_char], attrsonly: c_int) -> c_int;
}
