use std::io;
use std::str;
use std::default::Default;

use asnom::common;
use asnom::IResult;
use asnom::structures::{Tag, Integer, Sequence, ASNTag};

use asnom::parse::{parse_tag, parse_uint};
use asnom::write;

use tokio_core::io::{Codec, EasyBuf, Framed};
use tokio_proto::multiplex::RequestId;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

pub struct LDAPMessage {
    pub id: i32,
    pub protocolOP: Tag,
    // No Control yet
}

pub struct LDAPCodec;

impl Codec for LDAPCodec {
    type In = LDAPMessage;
    type Out = LDAPMessage;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<LDAPMessage>, io::Error> {
        match parse_tag(buf.as_slice()) {
            IResult::Incomplete(_) => Ok(None),
            IResult::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            IResult::Done(_, i) => {
                if let Some(mut tags) = i.match_id(16u64).and_then(|x| x.expect_constructed()) {
                    let protoop = tags.pop().unwrap()
                                    .match_class(common::TagClass::Application).unwrap();
                    let msgid: Vec<u8> = tags.pop().unwrap()
                                    .match_class(common::TagClass::Universal)
                                    .and_then(|x| x.match_id(2u64))
                                    .and_then(|x| x.expect_primitive()).unwrap();
                    if let IResult::Done(_, id) = parse_uint(msgid.as_slice()) {
                        return Ok(Some( LDAPMessage {
                            id: id as i32,
                            protocolOP: Tag::StructureTag(protoop),
                        }));
                    }
                }
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid LDAPMessage received."));
            }
        }
    }

    fn encode(&mut self, item: LDAPMessage, into: &mut Vec<u8>) -> io::Result<()> {
        let outtag = Tag::Sequence(Sequence {
            inner: vec![
                Tag::Integer(Integer {
                    inner: item.id as i64,
                    .. Default::default()
                }),
                item.protocolOP,
            ],
            .. Default::default()
        });

        try!(write::encode_into(into, outtag.into_structure()));

        Ok(())
    }
}
