use tokio_core::io::{Io, Codec, EasyBuf, Framed};
use std::io;

use futures::sync::mpsc;
use futures::{Future, Stream, Poll};

use tokio_proto::streaming::{Body, Message};
use tokio_proto::streaming::multiplex::{Frame, ClientProto, RequestId};

use asnom::common;
use asnom::IResult;
use asnom::structures::{Tag, Integer, Sequence, ASNTag};
use asnom::structure::StructureTag;

use asnom::parse::{parse_tag, parse_uint};
use asnom::write;

pub struct LdapCodec;

impl Codec for LdapCodec {
    type In = Frame<Tag, Tag, io::Error>;
    type Out = Frame<Tag, Tag, io::Error>;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
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
                        return match protoop.id {
                            // SearchResultEntry
                            4 => Ok(Some(Frame::Body {
                                id: id as u64,
                                chunk: Some(Tag::StructureTag(protoop)),
                            })),
                            // SearchResultDone
                            5 => Ok(Some(Frame::Body {
                                id: id as u64,
                                chunk: None,
                            })),
                            // Any other Message
                            _ => Ok(Some(Frame::Message {
                                id: id as u64,
                                message: Tag::StructureTag(protoop),
                                body: false,
                                solo: false,
                            })),
                        }
                    }
                }
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid (RequestId, Tag) received."));
            }
        }
    }

    fn encode(&mut self, msg: Self::Out, into: &mut Vec<u8>) -> io::Result<()> {
        match msg {
            Frame::Message {message, id, body, solo} => {
                let outtag = Tag::Sequence(Sequence {
                    inner: vec![
                        Tag::Integer(Integer {
                            inner: id as i64,
                            .. Default::default()
                        }),
                        message,
                    ],
                    .. Default::default()
                });

                let outstruct = outtag.into_structure();
                try!(write::encode_into(into, outstruct));
                Ok(())
            },
            _ => unimplemented!(),
        }
    }
}

pub struct LdapProto;

impl<T: Io + 'static> ClientProto<T> for LdapProto {
    type Request = Tag;
    type RequestBody = Tag;
    type Response = Tag;
    type ResponseBody = Tag;
    type Error = io::Error;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, LdapCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LdapCodec))
    }
}
