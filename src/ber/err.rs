use std::convert::From;
use std::error;
use std::fmt;
use std::io;

use byteorder;

pub enum Kind {
    IndefiniteLength,
    InvalidLengthEncoding,
    Io(io::Error),
    Byteorder(byteorder::Error),
}

pub struct Error {
    pub kind:   Kind,
    pub cause:  Option<Box<Error>>,
}

impl Error {
    pub fn new (kind: Kind, cause: Option<Box<Error>>) -> Error {
        Error {
            kind: kind,
            cause: cause,
        }
    }

    pub fn wrap (self, kind: Kind, ) -> Error {
        Error::new(kind, Some(Box::new(self)))
    }
}

impl fmt::Display for Error {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl fmt::Debug for Error {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description (&self) -> &str {
        match self.kind {
            Kind::IndefiniteLength => "Indefinite length is not allowed in LDAP according to RFC 2551 Section 5.1",
            Kind::InvalidLengthEncoding => "The long form of encoding type is not allowed for class Universal.",
            Kind::Io(ref x) => error::Error::description(x),
            Kind::Byteorder(ref x) => error::Error::description(x),
        }
    }

    fn cause (&self) -> Option<&error::Error> {
        match self.cause {
            Some(ref c) => Some(&**c),
            None => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from (err: io::Error) -> Error {
        Error {
            kind: Kind::Io(err),
            cause: None,
        }
    }
}
impl From<byteorder::Error> for Error {
    fn from (err: byteorder::Error) -> Error {
        Error {
            kind: Kind::Byteorder(err),
            cause: None,
        }
    }
}
