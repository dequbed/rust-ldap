//! Client optimized Event Queue

use std::io::Cursor;
use std::io::{Read, Write};

use mio;
use mio::{TryRead, TryWrite};
use mio::tcp::TcpStream;
use mio::util::Slab;

use protocol::ber::common::Tag;
use protocol::ber::encode;

struct Connection
{
    socket: TcpStream,
    //token: mio::Token,
    readbuf: Vec<u8>,
    writebuf: Vec<u8>,
}

impl mio::Handler for Connection
{
    type Timeout = ();
    type Message = (i32, Tag);

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Connection>, token: mio::Token, events: mio::EventSet)
    {
        if self.writebuf.len() > 0 && events.is_writable()
        {
            self.socket.write(self.writebuf.as_slice());
        }

        if events.is_readable()
        {
            self.socket.read_to_end(&mut self.readbuf);
        }
    }

    fn notify(&mut self, event_loop: &mut mio::EventLoop<Connection>, msg: Self::Message)
    {
        if let Ok(bytes) = encode(msg.1, msg.0)
        {
            self.writebuf.write(&bytes);
        }
    }
}
