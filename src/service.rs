use std::io;

use asnom::structures::Tag;

use futures::{Stream, Poll};

use tokio_proto::streaming::{Body, Message};

#[derive(Debug)]
pub enum LdapMessage {
    Once(Tag),
    Stream(Tag, LdapMessageStream),
}

#[derive(Debug)]
pub struct LdapMessageStream {
    inner: Body<Tag, io::Error>,
}

impl LdapMessageStream {
    pub fn empty() -> LdapMessageStream {
        LdapMessageStream {
            inner: Body::empty()
        }
    }
}

impl Stream for LdapMessageStream {
    type Item = Tag;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Tag>, io::Error> {
        self.inner.poll()
    }
}

pub type TokioMessage = Message<Tag, Body<Tag, io::Error>>;

impl From<TokioMessage> for LdapMessage {
    fn from(src: TokioMessage) -> Self {
        match src {
            Message::WithoutBody(tag) => LdapMessage::Once(tag),
            Message::WithBody(tag, body) =>
                LdapMessage::Stream(tag, LdapMessageStream { inner: body })
        }
    }
}

impl From<LdapMessage> for TokioMessage {
    fn from(src: LdapMessage) -> Self {
        match src {
            LdapMessage::Once(tag) => Message::WithoutBody(tag),
            LdapMessage::Stream(tag, body) => {
                let LdapMessageStream { inner } = body;
                Message::WithBody(tag, inner)
            }
        }
    }
}
