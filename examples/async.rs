extern crate tokio_core;
extern crate futures;
extern crate ldap;

use futures::Future;
use tokio_core::reactor::Core;
use ldap::Ldap;


fn main() {
    // TODO better error handling
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:389".parse().unwrap();

    core.run(futures::lazy(|| {
        Ldap::connect(&addr, &handle)
        .and_then(|ldap| {
            ldap.simple_bind("cn=root,dc=plabs".to_string(), "asdf".to_string())
        })
        .map(|res| {
            if res {
                println!("Bind succeeded!");
            } else {
                println!("Bind failed! :(");
            }
        })
    })).unwrap();
}
