use tag::LDAPTag;
use err::{LDAPResult, LDAPError};
use ber::{Tag, Class, Type, Payload};

pub struct Entry
{
    pub dn: String,
    pub attributes: Vec<Attribute>,
}

impl Entry
{
    pub fn from_tag(tag: Tag) -> LDAPResult<Entry>
    {
        if tag.is_class(Class::Application(4))
        {
            let mut payload = try!(tag.into_payload().into_inner_constructed());

            let dn_tag = try!(payload.remove(0).into_payload().into_inner_primitive());
            let dn = try!(String::from_utf8(dn_tag));

            let attr_tag = try!(payload.remove(1).into_payload().into_inner_constructed());
            let mut attributes = Vec::<Attribute>::new();
            for attr in attr_tag
            {
                let attrib = try!(Attribute::from_tag(attr));
                attributes.push(attrib);
            }

            return Ok(Entry{dn: dn, attributes: attributes});
        }

        Err(LDAPError::DecodingFailure)
    }
}

pub struct Attribute
{
    pub description: String,
    pub values: Vec<String>,
}

impl Attribute
{
    pub fn from_tag(tag: Tag) -> LDAPResult<Attribute>
    {
        let mut payload = try!(tag.into_payload().into_inner_constructed());

        let description = try!(payload.remove(0).into_payload().into_inner_primitive());
        let description = try!(String::from_utf8(description));

        let value_tags = try!(payload.remove(0).into_payload().into_inner_constructed());
        let mut values = Vec::<String>::new();
        for val in value_tags
        {
            let octets = try!(val.into_payload().into_inner_primitive());
            let value = try!(String::from_utf8(octets));
            values.push(value);
        }

        Ok(Attribute
        {
            description: description,
            values: values,
        })
    }
}

pub enum Scope
{
    BaseObject = 0,
    SingleLevel = 1,
    WholeSubtree = 2,
}

impl LDAPTag for Scope
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::Enumerated), Payload::Primitive(vec![self as u8]))
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let payload = try!(tag.into_payload().into_inner_primitive());
        match payload[0]
        {
            0 => Ok(Scope::BaseObject),
            1 => Ok(Scope::SingleLevel),
            2 => Ok(Scope::WholeSubtree),
            _ => Err(LDAPError::DecodingFailure),
        }
    }
}

pub enum DerefAlias
{
    NeverDerefAliases = 0,
    DerefInSearching = 1,
    DerefFindingBaseObj = 2,
    DerefAlways = 3,
}

impl LDAPTag for DerefAlias
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::Enumerated), Payload::Primitive(vec![self as u8]))
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let payload = try!(tag.into_payload().into_inner_primitive());
        match payload[0]
        {
            0 => Ok(DerefAlias::NeverDerefAliases),
            1 => Ok(DerefAlias::DerefInSearching),
            2 => Ok(DerefAlias::DerefFindingBaseObj),
            3 => Ok(DerefAlias::DerefAlways),
            _ => Err(LDAPError::DecodingFailure),
        }
    }
}

// TODO: Figure something out...
// pub enum Filter
// {
//     and(Vec<Filter>),
//     or(Vec<Filter>),
//     not(Filter),
//     equalityMatch(AttributeValueAssertion),
//     substrings(AttributeValueAssertion),
//     greaterOrEqual(AttributeValueAssertion),
//     lessOrEqual(AttributeValueAssertion),
//     present(String),
//     approxMatch(AttributeValueAssertion),
//     extensibleMatch(MatchingRuleAssertion),
// }

// impl LDAPTag for Filter
// {
//     fn into_tag(self) -> Tag
//     {
//         match self
//         {
//             Filter::and(filters) =>
//             {
//                 filters.into_tag().set_class(Class::ContextSpecific(0))
//             }
//         }
//     }
// }

pub struct ValueAssertion
{
    description: String,
    value: String,
}

impl LDAPTag for ValueAssertion
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::Sequence),
                 Payload::Constructed(vec![self.description.into_tag(), self.value.into_tag()]))
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let mut payload = try!(tag.into_payload().into_inner_constructed());

        let description: String = try!(LDAPTag::from_tag(payload.remove(0)));

        let value: String = try!(LDAPTag::from_tag(payload.remove(0)));

        Ok(ValueAssertion{
            description: description,
            value: value,
        })
    }
}

pub struct MatchingRuleAssertion
{
    pub rule: Option<String>,
    pub ruletype: Option<String>,
    pub value: String,
    pub attributes: Option<bool>,
}

pub enum Substrings
{
    Initial(String),
    Any(String),
    Final(String),
}

impl LDAPTag for Substrings
{
    fn into_tag(self) -> Tag
    {
        match self
        {
            Substrings::Initial(value) =>
                Tag::new(Class::ContextSpecific(0), Payload::Primitive(value.into_bytes())),
            Substrings::Any(value) =>
                Tag::new(Class::ContextSpecific(1), Payload::Primitive(value.into_bytes())),
            Substrings::Final(value) =>
                Tag::new(Class::ContextSpecific(2), Payload::Primitive(value.into_bytes())),
        }
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        match tag.class
        {
            Class::ContextSpecific(v) =>
            {
                let payload = try!(tag.into_payload().into_inner_primitive());
                let content: String = try!(String::from_utf8(payload));
                return match v
                {
                    0 => Ok(Substrings::Initial(content)),
                    1 => Ok(Substrings::Any(content)),
                    2 => Ok(Substrings::Final(content)),
                    _ => Err(LDAPError::DecodingFailure)
                }
            },
            _ => Err(LDAPError::DecodingFailure),
        }
    }
}
