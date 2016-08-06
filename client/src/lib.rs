//! LDAP Client Crate
//!
//! This crate serves as opinionated abstractions over `ldap_protocol`.
//! While it is useful for LDAP clients it will not work well as LDAP Server.

// TODO: Make this a deny.
#![warn(missing_docs)]

#[doc(no_inline)]
extern crate ldap_protocol as protocol;
extern crate mio;

use std::net::TcpStream;
use std::net::ToSocketAddrs;

use std::io::{Read, Write};

use protocol::ber::{self, common};
pub use protocol::Result;

pub mod bind;
mod queue;

/// Core LDAP struct
///
/// This struct contains all state of the LDAP connection this crate establishes.
/// It is used in all LDAP functions.
#[derive(Debug)]
pub struct LDAP
{
    // TODO: Later abstract over io::Read / io::Write (for LDAPS and LDAPI implementation)
    stream: TcpStream,

    msgid: i32,
}

impl LDAP
{

    /// Connect to the LDAP-Server found at `addr` using plain unencrypted TCP
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<LDAP>
    {
        let stream = try!(TcpStream::connect(addr));

        Ok(LDAP
        {
            stream: stream,
            msgid: 0,
        })
    }

    fn send(&mut self, tag: common::Tag) -> Result<()>
    {
        println!("Sending tag: {:?}", tag);
        let tagbuf = try!(ber::encode(tag, self.msgid));
        try!(self.stream.write(tagbuf.as_slice()));

        Ok(())
    }

    pub fn recv(&mut self) -> Result<common::Tag>
    {
        let mut buf = [0; 500];

        let readamount = try!(self.stream.read(&mut buf));
        println!("read: {}", readamount);

        let tag = try!(ber::decode(&mut buf));
        println!("Received tag: {:?}", tag);

        Ok(tag)
    }
}
