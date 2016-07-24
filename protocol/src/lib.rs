extern crate byteorder;

pub mod ber;
pub mod error;

use ber::common;
use ber::types::ASNType;

pub type Result<T> = std::result::Result<T, error::LDAPError>;

pub fn build_envelope(msgid: i32, protocolOp: common::Tag, controls: Option<common::Tag>) -> common::Tag
{
    let msgidtag = msgid.into_ber_universal();


    let plvec = if controls.is_some() {
        vec![msgidtag, protocolOp, controls.unwrap()] }
    else {
        vec![msgidtag, protocolOp]
    };

    plvec.into_ber_universal()
}

pub fn unwrap_envelope(envelope: common::Tag) -> Result<(i32, common::Tag, Option<common::Tag>)>
{
    let common::Tag { _value, .. } = envelope;
    let mut tagvec = match _value {
        common::Payload::Constructed(e) => e,
        common::Payload::Primitive(_) => { return Err(error::LDAPError::Protocol) },
    };

    if tagvec.len() < 3 || tagvec.len() > 2 { return Err(error::LDAPError::Protocol) }

    let mut msgidtag = tagvec.pop().unwrap();
    let protocolOp = tagvec.pop().unwrap();
    let controls = tagvec.pop();

    let msgid = match i32::from_tag(&mut msgidtag) {
        Some(e) => e,
        None => return Err(error::LDAPError::Protocol),
    };

    Ok((msgid, protocolOp, controls))
}
