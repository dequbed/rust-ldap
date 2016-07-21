extern crate byteorder;

pub mod ber;
pub mod bind;
pub mod error;

use std::net::{ToSocketAddrs, TcpStream};
use std::thread;

use std::sync::mpsc::{Sender, Receiver, channel};

use ber::common;

pub type Result<T> = std::result::Result<T, error::LDAPError>;

#[derive(Debug)]
pub struct LDAP
{
    // TODO: Later abstract over io::Read / io::Write
    stream: TcpStream,

    msgid: u32,

    encoder_channel: Sender<common::Tag>,
    decoder_channel: Receiver<common::Tag>,
}

impl LDAP
{
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<LDAP>
    {
        let stream = try!(TcpStream::connect(addr));
        let enc_str = try!(stream.try_clone());
        let dec_str = try!(stream.try_clone());

        let (encoder_tx, enc_rx) = channel();
        let (decoder_tx, dec_rx) = channel();


        println!("Spawning Encoder");
        let enc_child = thread::spawn(move || {
            let mut enc = ber::Encoder::from_writer_raw(enc_str);

            loop {
                let tag = enc_rx.recv().unwrap();
                println!("{:?}", &tag);
                enc.encode(&tag);
                enc.flush();
            }
        });

        println!("Spawning Decoder");
        let dec_child = thread::spawn(move || {
            let mut dec = ber::Decoder::from_reader_raw(dec_str);

            // loop {
            //     let tag = dec.decode().unwrap();
            //     decoder_tx.send(tag).unwrap();
            // }
        });

        Ok(LDAP
        {
            stream: stream,
            msgid: 0,
            encoder_channel: encoder_tx,
            decoder_channel: dec_rx,
        })
    }

    pub fn send(&self, tag: common::Tag) -> Result<()>
    {
        match self.encoder_channel.send(tag)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(error::LDAPError::Other)
        }
    }

    pub fn recv(&self) -> ()
    {
        let tag = self.decoder_channel.recv().unwrap();
        println!("{:?}", tag);
    }
}
