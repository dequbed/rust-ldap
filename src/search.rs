use std::io;
use std::collections::HashMap;

use asnom::structures::{Tag, Sequence, Integer, OctetString, Boolean};

use asnom::common::TagClass::*;

use rfc4515::parse;

use futures::{Future, stream, Stream};
use tokio_service::Service;

use ldap::Ldap;
use service::LdapMessage;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scope {
    BaseObject   = 0,
    SingleLevel  = 1,
    WholeSubtree = 2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DerefAliases {
    Never             = 0,
    InSearch          = 1,
    FindingBaseObject = 2,
    Always            = 3,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchEntry {
    Reference(Vec<String>),
    Object {
        object_name: String,
        attributes: HashMap<String, Vec<String>>,
    },
}

impl SearchEntry {
    pub fn construct(_: Tag) -> SearchEntry {
        SearchEntry::Reference(Vec::new())
    }
}

impl Ldap {
    pub fn search(&self,
                    base: String,
                    scope: Scope,
                    deref: DerefAliases,
                    typesonly: bool,
                    filter: String) ->
        Box<Future<Item = Vec<SearchEntry>, Error = io::Error>> {
        let req = Tag::Sequence(Sequence {
            id: 3,
            class: Application,
            inner: vec![
                   Tag::OctetString(OctetString {
                       inner: base.into_bytes(),
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: scope as i64,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: deref as i64,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: 0,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: 0,
                       .. Default::default()
                   }),
                   Tag::Boolean(Boolean {
                       inner: typesonly,
                       .. Default::default()
                   }),
                   parse(&filter).unwrap(),
                   Tag::Sequence(Sequence {
                       .. Default::default()
                   })
            ],
        });

        let fut = self.call(req).and_then(|res| {
            match res {
                LdapMessage::Stream(first, body) => {
                    let fstr = stream::once(Ok(first));
                    let ostr = fstr.chain(body);

                    ostr.map(|x| SearchEntry::construct(x))
                        .collect()
                        .and_then(|x| Ok(x))
                },
                _ => unimplemented!(),
            }
        });

        Box::new(fut)
    }
}

