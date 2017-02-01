use tokio_core::io::{Io, Codec, EasyBuf, Framed};
use tokio_proto::multiplex::{RequestId, ServerProto, ClientProto};
use std::io;

use asnom::common;
use asnom::IResult;
use asnom::structures::{Tag, Integer, Sequence, ASNTag};

use asnom::parse::{parse_tag, parse_uint};
use asnom::write;

pub struct LdapCodec;
pub struct LdapProto;

impl Codec for LdapCodec {
    type In = (RequestId, Tag);
    type Out = (RequestId, Tag);

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<(RequestId, Tag)>, io::Error> {
        match parse_tag(buf.as_slice()) {
            IResult::Incomplete(_) => { Ok(None)},
            IResult::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            IResult::Done(_, i) => {
                if let Some(mut tags) = i.match_id(16u64).and_then(|x| x.expect_constructed()) {
                    let protoop = tags.pop().unwrap();
                    let msgid: Vec<u8> = tags.pop().unwrap()
                                    .match_class(common::TagClass::Universal)
                                    .and_then(|x| x.match_id(2u64))
                                    .and_then(|x| x.expect_primitive()).unwrap();
                    if let IResult::Done(_, id) = parse_uint(msgid.as_slice()) {
                        return Ok(Some((id as RequestId, Tag::StructureTag(protoop))));
                    }
                }
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid (RequestId, Tag) received."));
            }
        }
    }

    fn encode(&mut self, item: (RequestId, Tag), into: &mut Vec<u8>) -> io::Result<()> {
        let (id, protocol_op) = item;
        let outtag = Tag::Sequence(Sequence {
            inner: vec![
                Tag::Integer(Integer {
                    inner: id as i64,
                    .. Default::default()
                }),
                protocol_op,
            ],
            .. Default::default()
        });

        let outstruct = outtag.into_structure();
        try!(write::encode_into(into, outstruct));
        Ok(())
    }
}

impl<T: Io + 'static> ClientProto<T> for LdapProto {
    type Request = Tag;
    type Response = Tag;

    type Transport = Framed<T, LdapCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LdapCodec))
    }
}

impl<T: Io + 'static> ServerProto<T> for LdapProto {
    type Request = Tag;
    type Response = Tag;

    type Transport = Framed<T, LdapCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LdapCodec))
    }
}
