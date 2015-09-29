use tag::LDAPTag;
use ber::{Tag, Class, Type, Payload};

pub struct Entry
{
    dn: String,
    attributes: Vec<Attribute>,
}

impl Entry
{
    pub fn from_tag(tag: Tag) -> Option<Entry>
    {
        if tag.is_class(Class::Application(4))
        {
            let payload = tag.into_payload().into_inner_constructed();
            if payload.is_none() { return None; }
            let mut payload = payload.unwrap();
            // String tag
            let dn_tag = payload.remove(0).into_payload().into_inner_primitive();
            if dn_tag.is_none() { return None; }
            let dn = String::from_utf8(dn_tag.unwrap());
            if dn.is_err() { return None; }
            // Sequence tag
            let attr_tag = payload.remove(1).into_payload().into_inner_constructed();
            if attr_tag.is_none() { return None; }
            let attr_tag = attr_tag.unwrap();
            let mut attributes = Vec::<Attribute>::new();
            for attr in attr_tag
            {
                let attrib = Attribute::from_tag(attr);
                if attrib.is_none() { return None; }
                attributes.push(attrib.unwrap());
            }
            return Some(Entry{dn: dn.unwrap(), attributes: attributes});
        }

        None
    }
}

pub struct Attribute
{
    description: String,
    values: Vec<String>,
}

impl Attribute
{
    pub fn from_tag(tag: Tag) -> Option<Attribute>
    {
        let mut payload = tag.into_payload().into_inner_constructed();
        if payload.is_none() { return None; }
        let mut payload = payload.unwrap();

        let description = payload.remove(0).into_payload().into_inner_primitive();
        if description.is_none() { return None; }
        let description = String::from_utf8(description.unwrap());
        if description.is_err() { return None; }

        let value_tags = payload.remove(1).into_payload().into_inner_constructed();
        if value_tags.is_none() { return None; }

        let mut values = Vec::<String>::new();
        for val in value_tags.unwrap()
        {
            let octets = val.into_payload().into_inner_primitive();
            if octets.is_none() { return None; }
            let value = String::from_utf8(octets.unwrap());
            if value.is_err() { return None; }
            values.push(value.unwrap());
        }

        Some(Attribute
        {
            description: description.unwrap(),
            values: values,
        })
    }
}

pub enum Scope
{
    baseObject = 0,
    singleLevel = 1,
    wholeSubtree = 2,
}

impl LDAPTag for Scope
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::Enumerated), Payload::Primitive(vec![self as u8]))
    }
}

pub enum derefAlias
{
    neverDerefAliases = 0,
    derefInSearching = 1,
    derefFindingBaseObj = 2,
    derefAlways = 3,
}

impl LDAPTag for derefAlias
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::Enumerated), Payload::Primitive(vec![self as u8]))
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
}

pub struct MatchingRuleAssertion
{
    rule: Option<String>,
    ruletype: Option<String>,
    value: String,
    attributes: Option<bool>,
}

pub enum Substrings
{
    initial(String),
    any(String),
    finalval(String),
}

impl LDAPTag for Substrings
{
    fn into_tag(self) -> Tag
    {
        match self
        {
            Substrings::initial(value) =>
                Tag::new(Class::ContextSpecific(0), Payload::Primitive(value.into_bytes())),
            Substrings::any(value) =>
                Tag::new(Class::ContextSpecific(1), Payload::Primitive(value.into_bytes())),
            Substrings::finalval(value) =>
                Tag::new(Class::ContextSpecific(2), Payload::Primitive(value.into_bytes())),
        }
    }
}
