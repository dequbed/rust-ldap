//! LDAP Crate
//!
//! This crate is a meta-crate. It's sole purpose is to re-export symbols from
//! its subcrates.
//!
//! `ldap_protocol` implements structures according to RFC4511. You will most
//! likely never use this crate directly but use the abstractions offered by
//! ldap_client or ldap_server.
//!
//! `ldap_client` is an abstraction over the pure protocol functions. If you
//! want to make your project LDAP-Aware you will most likely want to use that
//! crate instead.


extern crate ldap_protocol as protocol;
extern crate ldap_client as client;
