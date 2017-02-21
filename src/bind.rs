use std::io;

use asnom::structures::{Tag, Sequence, Integer, OctetString};
use asnom::structure::StructureTag;

use asnom::common::TagClass::*;

use asnom::structures::ASNTag;

use futures::Future;
use tokio_service::Service;

use ldap::Ldap;
use service::LdapMessage;

impl Ldap {
    pub fn simple_bind(&self, dn: String, pw: String) ->
        Box<Future<Item = bool, Error = io::Error>> {
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

        let fut = self.call(req).and_then(|res|
            match res {
                LdapMessage::Once(Tag::StructureTag(tag)) => {
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
}
