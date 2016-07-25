//! Client optimized Event Queue

use std::io::Cursor;

use mio;
use mio::{TryRead, TryWrite};
use mio::tcp::TcpStream;
use mio::util::Slab;

struct Connection
{
    socket: TcpStream,
    token: mio::Token,
    state: State,
}

impl mio::Handler for Connection
{
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Connection>, token: mio::Token, events: mio::EventSet)
    {

        match self.state
        {
            State::Reading(_) =>
            {
                self.read(event_loop)
            },
            State::Writing(_) =>
            {
                self.write(event_loop)
            },
            State::Closed =>
            {
                event_loop.shutdown();
            }
        }
    }
}

impl Connection
{
    fn read(&mut self, event_loop: &mut mio::EventLoop<Connection>)
    {
        match self.socket.try_read_buf(self.state.mut_read_buf())
        {
            Ok(Some(0)) =>
            {
                // Socket is closed
                self.state = State::Closed;
            },
            Ok(Some(n)) =>
            {
                self.reregister(event_loop)
            },
            Ok(None) =>
            {
                self.reregister(event_loop)
            }
            Err(e) =>
            {
                panic!("Got an error: {:?}", e);
            },
        }
    }

    fn write(&mut self, event_loop: &mut mio::EventLoop<Connection>)
    {

    }

    fn reregister(&mut self, event_loop: &mut mio::EventLoop<Connection>)
    {

    }
}

enum State
{
    Reading(Vec<u8>),
    Writing(Cursor<Vec<u8>>),
    Closed,
}

impl State
{
    fn mut_read_buf(&mut self) -> &mut Vec<u8>
    {
        match *self
        {
            State::Reading(ref mut buf) => buf,
            // FIXME
            _ => panic!()
        }
    }
}
