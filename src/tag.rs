use ber::{Tag, Type, Class, Payload};

pub trait LDAPTag
{
    fn into_tag(self) -> Tag;
}

impl LDAPTag for Tag
{
    fn into_tag(self) -> Tag
    {
        self
    }
}

impl<T> LDAPTag for Vec<T> where T: LDAPTag
{
    fn into_tag(self) -> Tag
    {
        let tagvec = self.into_iter().map(|x| x.into_tag()).collect();
        Tag::new(Class::Universal(Type::Sequence), Payload::Constructed(tagvec))
    }
}

impl<T> LDAPTag for Box<T> where T: LDAPTag
{
    fn into_tag(self) -> Tag
    {
        (*self).into_tag()
    }
}

impl LDAPTag for String
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::OctetString), Payload::Primitive(self.into_bytes()))
    }
}

impl LDAPTag for bool
{
    fn into_tag(self) -> Tag
    {
        let pl: u8;
        if self {pl = 0xFF} else {pl = 0x00}
        Tag::new(Class::Universal(Type::Boolean), Payload::Primitive(vec![pl]))
    }
}

impl LDAPTag for i32
{
    fn into_tag(self) -> Tag
    {
        let mut count: u8 = 0;
        let mut clone = self;
        while {count += 1; clone >>= 8; clone > 0 }{}
        let mut bytes = Vec::<u8>::new();
        for i in (0..count).rev()
        {
            bytes.push(((self & (0xFF << i * 8)) >> i * 8) as u8);
        }
        Tag::new(Class::Universal(Type::Integer), Payload::Primitive(bytes))
    }
}
