use std::io;
use std::net::SocketAddr;

use tokio_core::io::Io;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

use service::LdapService;

pub struct Ldap<T> {
    transport: LdapService<T>,
}

pub struct LdapSync<T> {
    inner: Ldap<T>,
    core: Core
}

impl<T: Io> Ldap<T> {
    pub fn new(transport: T) -> Ldap<T> {
        Ldap { transport: transport }
    }
}

impl LdapSync<TcpStream> {
    pub fn connect(addr: &SocketAddr) -> Result<LdapSync<TcpStream>, io::Error> {
        // TODO better error handling
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let client_fut = TcpStream::connect(addr, handle);

        let stream = try!(core.run(client_fut));

        LdapSync::<TcpStream> {
            inner:  Ldap::new(stream),
            core: core,
        }
    }
}
