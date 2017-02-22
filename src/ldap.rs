use std::io;
use std::net::SocketAddr;

use asnom::structures::Tag;

use futures::Future;

use tokio_core::reactor::Handle;

use tokio_service::Service;

use tokio_proto::util::client_proxy::ClientProxy;
use tokio_proto::TcpClient;

use protocol::LdapProto;
use service::{LdapMessage, TokioMessage};

pub struct Ldap {
    inner: ClientTypeMap<ClientProxy<TokioMessage, TokioMessage, io::Error>>,
}

impl Ldap {
    pub fn connect(addr: &SocketAddr, handle: &Handle) ->
        Box<Future<Item = Ldap, Error = io::Error>> {
        let ret = TcpClient::new(LdapProto)
            .connect(addr, handle)
            .map(|client_proxy| {
                let typemap = ClientTypeMap { inner: client_proxy };
                Ldap { inner: typemap }
            });
        Box::new(ret)
    }
}

impl Service for Ldap {
    type Request = Tag;
    type Response = LdapMessage;
    type Error = io::Error;
    type Future = Box<Future<Item = LdapMessage, Error = io::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(LdapMessage::Once(req))
    }
}

struct ClientTypeMap<T> {
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
