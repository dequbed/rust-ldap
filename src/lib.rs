extern crate asnom;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;

extern crate byteorder;

#[macro_use]
extern crate log;

mod codec;

use std::io;
use tokio_core::reactor::Core;

struct LDAP {
    reactor: Core,
}

impl LDAP {
    pub fn new() -> io::Result<LDAP> {
        let core = try!(Core::new());

        Ok(LDAP { reactor: core })
    }
}
