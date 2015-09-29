use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::string;

use byteorder;

pub type LDAPResult<Value> = Result<Value, LDAPError>;

pub enum LDAPError
{
    DecodingFailure,
    IndefiniteLength,
    InvalidLengthEncoding,
    Io(io::Error),
    Byteorder(byteorder::Error),
    UTF8Error(string::FromUtf8Error),
}

impl fmt::Display for LDAPError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:?}", *self)
    }
}

impl fmt::Debug for LDAPError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Error: {}", error::Error::description(self))
    }
}

impl error::Error for LDAPError
{
    fn description(&self) -> &str
    {
        match *self
        {
            LDAPError::DecodingFailure =>
                "Decoding failure, input is not valid in this situation.",
            LDAPError::IndefiniteLength =>
                "Indefinite length is not allowed in LDAP according to RFC 2551 Section 5.1",
            LDAPError::InvalidLengthEncoding =>
                "The long form of encoding type is not allowed for class Universal.",
            LDAPError::UTF8Error(ref x) =>
                error::Error::description(x),
            LDAPError::Io(ref x) =>
                error::Error::description(x),
            LDAPError::Byteorder(ref x) =>
                error::Error::description(x),
        }
    }
}

impl From<io::Error> for LDAPError
{
    fn from(err: io::Error) -> LDAPError
    {
        LDAPError::Io(err)
    }
}
impl From<byteorder::Error> for LDAPError
{
    fn from(err: byteorder::Error) -> LDAPError
    {
        LDAPError::Byteorder(err)
    }
}
impl From<string::FromUtf8Error> for LDAPError
{
    fn from(err: string::FromUtf8Error) -> LDAPError
    {
        LDAPError::UTF8Error(err)
    }
}
