
use std::fmt;
use std::mem;

use libc::{c_char, c_int, c_ulong, c_void};

pub static LDAP_VERSION1: isize = 1;
pub static LDAP_VERSION2: isize = 2;
pub static LDAP_VERSION3: isize = 3;

pub static LDAP_OPT_PROTOCOL_VERSION: isize = 0x0011;

pub static LDAP_SCOPE_BASE: isize = 0x0000;
pub static LDAP_SCOPE_BASEOBJECT: isize = 0x0000;
pub static LDAP_SCOPE_ONELEVEL: isize = 0x0001;
pub static LDAP_SCOPE_ONE: isize = 0x0001;
pub static LDAP_SCOPE_SUBTREE: isize = 0x0002;
pub static LDAP_SCOPE_SUB: isize = 0x0002;
pub static LDAP_SCOPE_SUBORDINATE: isize = 0x0003;
pub static LDAP_SCOPE_CHILDREN: isize = 0x0003;
pub static LDAP_SCOPE_DEFAULT: isize = -1;

pub static LDAP_SUCCESS: i32 = 0x00;

#[link(name = "ldap")]
extern
{
    pub fn ldap_initialize(ld: *mut *mut LDAP, url: *const c_char) -> c_int;
    pub fn ldap_simple_bind_s(ld: *mut LDAP, who: *const c_char, passwd: *const c_char) -> c_int;
    pub fn ldap_unbind_s(ld: *mut LDAP) -> c_int;

    pub fn ldap_err2string(err: c_int) -> *mut c_char;

    pub fn ldap_set_option(ld: *mut LDAP, option: c_int, invalue: *const c_void) -> c_int;

    pub fn ldap_search_s(ld: *mut LDAP,
                         base: *const c_char,
                         scope: c_int,
                         filter: *const c_char,
                         attrs: *const *const c_char,
                         attrsonly: c_int,
                         res: *mut *mut LDAPMessage) -> c_int;
    pub fn ldap_search_ext_s(ld: *mut LDAP,
                             base: *const c_char,
                             scope: c_int,
                             filter: *const c_char,
                             attrs: *const *const c_char,
                             attrsonly: c_int,
                             serverctls: *mut *mut LDAPControl,
                             clientctrls: *mut *mut LDAPControl,
                             timeout: *mut timeval,
                             sizelimit: c_int,
                             res: *mut *mut LDAPMessage) -> c_int;

    pub fn ldap_count_entries(ld: *mut LDAP, msg: *mut LDAPMessage) -> c_int;
    pub fn ldap_first_entry(ld: *mut LDAP, msg: *mut LDAPMessage) -> *mut LDAPMessage;
    pub fn ldap_next_entry(ld: *mut LDAP, msg: *mut LDAPMessage) -> *mut LDAPMessage;

    pub fn ldap_count_values_len(vals: *mut *mut berval) -> c_int;
    pub fn ldap_get_values_len(ld: *mut LDAP, entry: *mut LDAPMessage, target: *const c_char) -> *mut *mut berval;

    pub fn ldap_get_dn(ld: *mut LDAP, msg: *mut LDAPMessage) -> *const c_char;

    pub fn ldap_memfree(ld: *mut LDAP);
    pub fn ldap_msgfree(msg: *mut LDAPMessage);
    pub fn ldap_value_free(val: *mut *const c_char);
}

pub enum LDAP { }
pub enum timeval { }
pub enum LDAPMessage { }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LDAPControl
{
    pub ldctl_oid: *mut c_char,
    pub ldctl_value: berval,
    pub ldctl_iscritical: c_char,
}
impl ::std::default::Default for LDAPControl
{
    fn default() -> LDAPControl { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct berval
{
    pub bv_len: c_ulong,
    pub bv_val: *mut c_char,
}
impl ::std::default::Default for berval
{
    fn default() -> berval { unsafe { mem::zeroed() } }
}

