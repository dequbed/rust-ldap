use tokio_core::io::{Io, Codec, EasyBuf, Framed};
use std::io;
use std::collections::HashSet;

use tokio_proto::streaming::multiplex::{Frame, ClientProto};

use asnom::common;
use asnom::IResult;
use asnom::structures::{Tag, Integer, Sequence, ASNTag};

use asnom::parse::{parse_tag, parse_uint};
use asnom::write;

pub struct LdapCodec {
    search_seen: HashSet<u64>,
}

impl Codec for LdapCodec {
    type In = Frame<Tag, Tag, io::Error>;
    type Out = Frame<Tag, Tag, io::Error>;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let l = buf.len();
        match parse_tag(buf.drain_to(l).as_slice()) {
            IResult::Incomplete(_) => { Ok(None)},
            IResult::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            IResult::Done(_, i) => {
                trace!("Received packet: {:?}", i);
                if let Some(mut tags) = i.match_id(16u64).and_then(|x| x.expect_constructed()) {
                    let protoop = tags.pop().unwrap();
                    let msgid: Vec<u8> = tags.pop().unwrap()
                                    .match_class(common::TagClass::Universal)
                                    .and_then(|x| x.match_id(2u64))
                                    .and_then(|x| x.expect_primitive()).unwrap();
                    if let IResult::Done(_, id) = parse_uint(msgid.as_slice()) {
                        debug!("Received Message with ID {} and ProtocolOP {:?}", id, &protoop);
                        return match protoop.id {
                            // SearchResultEntry
                            4 => {
                                // We have already received the first of those results, so we only
                                // send a body frame.
                                if self.search_seen.contains(&id) {
                                    Ok(Some(Frame::Body {
                                        id: id as u64,
                                        chunk: Some(Tag::StructureTag(protoop)),
                                    }))
                                } // If we haven't yet seen that search, we need to initially send a whole message
                                else {
                                    self.search_seen.insert(id);
                                    Ok(Some(Frame::Message {
                                        id: id as u64,
                                        message: Tag::StructureTag(protoop),
                                        body: true,
                                        solo: false,
                                    }))
                                }
                            },
                            // SearchResultDone
                            5 => {
                                self.search_seen.remove(&id);
                                Ok(Some(Frame::Body {
                                    id: id as u64,
                                    chunk: None,
                                }))
                            },
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
            Frame::Message {message, id, body: _, solo: _} => {
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
                trace!("Sending packet: {:?}", &outstruct);
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
        let ldapcodec = LdapCodec { search_seen: HashSet::new() };
        Ok(io.framed(ldapcodec))
    }
}
