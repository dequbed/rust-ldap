use std::io;

use asnom::structures::Tag;

use futures::{Future, Stream, Poll};
use futures::sync::mpsc;

use tokio_proto::streaming::{Body, Message};
use tokio_proto::util::client_proxy::ClientProxy;
use tokio_service::Service;

use protocol::LdapCodec;

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
    pub fn pair() -> (mpsc::Sender<Result<Tag, io::Error>>, LdapMessageStream) {
        let (tx, rx) = Body::pair();
        (tx, LdapMessageStream { inner: rx })
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

pub struct ClientTypeMap<T> {
    inner: T
}

impl<T> Service for ClientTypeMap<T>
    where T: Service<Request = TokioMessage, Response = TokioMessage, Error = io::Error>,
          T::Future: 'static {
    type Request = LdapMessage;
    type Response = LdapMessage;
    type Error = io::Error;
    type Future = Box<Future<Item = LdapMessage, Error = io::Error>>;

    fn call(&self, req: LdapMessage) -> Self::Future {
        Box::new(self.inner.call(req.into()).map(LdapMessage::from))
    }
}
