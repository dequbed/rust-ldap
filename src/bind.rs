use prelude::*;
use ber::common;

use result::LDAPResult;

pub struct BindResponse
{
    result: LDAPResult,
    sasl_credentials: Option<String>,
}

/// Synchronous bind (only simple auth currently)
pub fn ldap_bind_s(ld: &mut LDAP, dn: LDAPDN, password: String)
{
    let versiontag = {
        let class = common::Class::Universal(common::UniversalTypes::Integer);
        let pl = common::Payload::Primitive(vec![0x03]);

        common::construct(class, pl)
    };

    let nametag = {
        let class = common::Class::Universal(common::UniversalTypes::OctetString);
        let pl = common::Payload::Primitive(dn.into_bytes());

        common::construct(class, pl)
    };

    let authtag = {
        let class = common::Class::ContextSpecific(0);
        let pl = common::Payload::Primitive(password.into_bytes());

        common::construct(class, pl)
    };

    let bindrequest = {
        let class = common::Class::Application(0);
        let pl = common::Payload::Constructed(vec![versiontag, nametag, authtag]);

        common::construct(class, pl)
    };

    ld.send(bindrequest);
}

pub fn ldap_unbind(ld: &mut LDAP)
{
    let unbindrequest = {
        let class = common::Class::Application(2);
        let pl = common::Payload::Primitive(Vec::new());

        common::construct(class, pl)
    };

    ld.send(unbindrequest);
}
