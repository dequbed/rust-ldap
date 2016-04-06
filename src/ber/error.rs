use std::convert::From;
use std::{error, fmt, io, string};
use byteorder;

pub enum ASN1Error
{
    IndefiniteLength,
    InvalidLenght,
    InvalidASN1,
    ExtendedTagTooLong,
    Io(io::Error),
}

impl fmt::Display for ASN1Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:?}", *self)
    }
}

impl fmt::Debug for ASN1Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Error: {}", error::Error::description(self))
    }
}

impl error::Error for ASN1Error
{
    fn description(&self) -> &str
    {
        match *self
        {
            ASN1Error::InvalidASN1 =>
                "Invalid BER Structures detected.",
            ASN1Error::IndefiniteLength =>
                "Indefinite Length is not valid for LDAP. (RFC 2551 Section 5.1)",
            ASN1Error::InvalidLenght =>
                "The long encoding form for length bytes is not valid for Universal tags.",
            ASN1Error::ExtendedTagTooLong =>
                "Rust-LDAP currently only handles extended tags up to 2^64. If you hit this case *please* open an issue.",
            ASN1Error::Io(ref x) =>
                error::Error::description(x),
        }
    }
}

impl From<io::Error> for ASN1Error
{
    fn from(err: io::Error) -> ASN1Error
    {
        ASN1Error::Io(err)
    }
}
