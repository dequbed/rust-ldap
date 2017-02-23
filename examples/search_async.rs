extern crate tokio_core;
extern crate ldap;

use tokio_core::reactor::{Core, Handle};
use ldap::Ldap;

pub fn main() {
    let addr = "127.0.0.1:389".parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let ldap = core.run(Ldap::connect(&addr, &handle)).unwrap();
    let bind = core.run(ldap.simple_bind("cn=root,dc=plabs".to_string(), "asdf".to_string()));

    let search_results = core.run(ldap.search("dc=plabs".to_string(),
                                  ldap::Scope::WholeSubtree,
                                  ldap::DerefAliases::Never,
                                  false,
                                  "(objectClass=*)".to_string()));

    println!("Search Results: {:?}", search_results)
}
