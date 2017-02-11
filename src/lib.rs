extern crate asnom;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

extern crate byteorder;

#[macro_use]
extern crate log;

use tokio_service::Service;

use std::ops::Drop;
use protocol::LdapProto;

use std::io;
use std::default::Default;
use std::net::SocketAddr;

use futures::Future;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpStream;
use tokio_proto::TcpClient;
use tokio_proto::multiplex::ClientService;

use asnom::structures::*;
use asnom::common::TagClass::*;

mod protocol;

pub struct LDAP {
    inner: ClientService<TcpStream, LdapProto>,
    core: Core
}

impl LDAP {
    pub fn new(addr: &SocketAddr) -> Result<LDAP, io::Error> {
        let mut core = Core::new().unwrap();

        let handle = core.handle();

        let client_fut = TcpClient::new(LdapProto).connect(addr, &handle);

        let clientres = core.run(client_fut);

        match clientres {
            Ok(client) => {
                Ok(LDAP {
                    inner: client,
                    core: core,
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn simple_bind(&self, dn: String, pw: String) -> Box<Future<Item = bool, Error = io::Error>> {
        let req = Tag::Sequence(Sequence {
            id: 0,
            class: Application,
            inner: vec![
                   Tag::Integer(Integer {
                       inner: 3,
                       .. Default::default()
                   }),
                   Tag::OctetString(OctetString {
                       inner: dn.into_bytes(),
                       .. Default::default()
                   }),
                   Tag::OctetString(OctetString {
                       id: 0,
                       class: Context,
                       inner: pw.into_bytes(),
                   })
            ],
        });

        let fut = self.inner.call(req).and_then(|res|
            match res {
                Tag::StructureTag(tag) => {
                    if let Some(i) = tag.expect_constructed() {
                        return Ok(i[0] == Tag::Integer(Integer {
                            id: 10,
                            class: Universal,
                            inner: 0
                        }).into_structure())
                    } else {
                        return Ok(false)
                    }
                }
                _ => unimplemented!(),
            }
        );
        Box::new(fut)
    }

    pub fn simple_bind_s(&mut self, dn: String, pw: String) -> Result<bool,io::Error> {
        let fut = self.simple_bind(dn, pw);
        self.core.run(fut)
    }
}

impl Drop for LDAP {
    fn drop(&mut self) {
        let _ = self.core.run(self.inner.call(Tag::Null(Null {
            id: 2,
            class: Application,
            inner: (),
        })));
    }
}
