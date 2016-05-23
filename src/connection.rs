use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};

use std::io::{BufReader, BufWriter};

use ber::{self, common};
use ber::types::ASNType;
use Result;

pub struct LDAP
{
    stream: TcpStream,
    msgid: i32,
    enc: ber::Encoder<BufWriter<TcpStream>>,
    dec: ber::Decoder<BufReader<TcpStream>>,
}

impl LDAP
{
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<LDAP>
    {
        let stream = try!(TcpStream::connect(addr));
        let enc = ber::Encoder::from_writer(try!(stream.try_clone()));
        let dec = ber::Decoder::from_reader(try!(stream.try_clone()));

        Ok(LDAP
        {
            stream: stream,
            enc: enc,
            dec: dec,
            msgid: 0,
        })
    }

    pub fn send(&mut self, mut tag: common::Tag) -> Result<()>
    {
        let messageid = self.msgid.into_ber_universal();

        let tagvec = vec![messageid, tag];

        try!(self.enc.encode(&tagvec.into_ber_universal()));

        self.enc.flush();

        Ok(())
    }

    pub fn read(&mut self) -> Result<common::Tag>
    {
        let tag = try!(self.dec.decode());

        println!("{:?}", &tag);

        Ok(tag)
    }
}
