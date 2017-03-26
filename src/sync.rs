use std::io;
use std::net::SocketAddr;

use ldap::Ldap;
use search::{Scope, DerefAliases, SearchEntry};

use tokio_core::reactor::{Core, Handle};

pub struct LdapSync {
    inner: Ldap,
    core: Core,
}

impl LdapSync {
    pub fn connect(addr: &SocketAddr) -> Result<LdapSync, io::Error> {
        // TODO better error handling
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let ldapfut = Ldap::connect(addr, &handle);
        let ldap = try!(core.run(ldapfut));

        Ok(LdapSync { inner: ldap, core: core })
    }

    pub fn connect_ssl(addr: &str) -> Result<LdapSync, io::Error> {
        // TODO better error handling
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let ldapfut = Ldap::connect_ssl(addr, &handle);
        let ldap = try!(core.run(ldapfut));

        Ok(LdapSync { inner: ldap, core: core })
    }

    pub fn simple_bind(&mut self, dn: String, pw: String) -> io::Result<bool> {
        self.core.run(self.inner.simple_bind(dn, pw))
    }

    pub fn search(&mut self,
                  base: String,
                  scope: Scope,
                  deref: DerefAliases,
                  typesonly: bool,
                  filter: String,
                  attrs: Vec<String>) -> io::Result<Vec<SearchEntry>> {
        self.core.run(self.inner.search(base, scope, deref, typesonly, filter, attrs))
    }
}
