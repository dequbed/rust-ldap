extern crate asnom;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

extern crate byteorder;

#[macro_use]
extern crate log;

mod protocol;

use protocol::LdapProto;

use std::default::Default;
use futures::Future;
use tokio_core::reactor::{Core, Handle};
use tokio_proto::TcpClient;
use tokio_proto::multiplex::Multiplex;
use tokio_service::Service;

use asnom::structures::*;
use asnom::common::TagClass::*;

pub fn main() {
    let mut core = Core::new().unwrap();

    let addr = "127.0.0.1:389".parse().unwrap();

    let handle = core.handle();

    let client = TcpClient::new(LdapProto).connect(&addr, &handle);

    let req = Tag::Sequence(Sequence {
        id: 0,
        class: Application,
        inner: vec![
               Tag::Integer(Integer {
                   inner: 3,
                   .. Default::default()
               }),
               Tag::OctetString(OctetString {
                   inner: String::from("cn=root,dc=plabs").into_bytes(),
                   .. Default::default()
               }),
               Tag::OctetString(OctetString {
                   id: 0,
                   class: Context,
                   inner: String::from("asdf").into_bytes(),
               })
        ],
    });

    let response = client.and_then(|c| c.call(req)
                    .and_then(move |response| {
                        println!("CLIENT: {:?}", response);
                        Ok(())
                    }));

    let a = core.run(
        response
    ).unwrap();
}
