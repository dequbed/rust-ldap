use std::convert::From;
use std::{error, io, fmt};

use ber::error::ASN1Error;

pub enum LDAPError
{
    ASN1(ASN1Error),
    Io(io::Error),
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
            LDAPError::ASN1(ref x) => error::Error::description(x),
            LDAPError::Io(ref x) => error::Error::description(x),
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

impl From<ASN1Error> for LDAPError
{
    fn from(err: ASN1Error) -> LDAPError
    {
        match err
        {
            ASN1Error::Io(e) => LDAPError::from(e),
            _ => LDAPError::ASN1(err),
        }
    }
}
