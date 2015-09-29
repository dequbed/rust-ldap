use LDAPResult;
use ber::{Tag, Type, Class, Payload};

pub trait LDAPTag
{
    fn into_tag(self) -> Tag;
    fn from_tag(tag: Tag) -> LDAPResult<Self>;
}

impl LDAPTag for Tag
{
    fn into_tag(self) -> Tag
    {
        self
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        Ok(tag)
    }
}

impl<T> LDAPTag for Vec<T> where T: LDAPTag
{
    fn into_tag(self) -> Tag
    {
        let tagvec = self.into_iter().map(|x| x.into_tag()).collect();
        Tag::new(Class::Universal(Type::Sequence), Payload::Constructed(tagvec))
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let tags = try!(tag.into_payload().into_inner_constructed());
        let mut values = Vec::<T>::new();
        for tag in tags
        {
            let value: T = try!(LDAPTag::from_tag(tag));
            values.push(value);
        }
        Ok(values)
    }
}

impl LDAPTag for String
{
    fn into_tag(self) -> Tag
    {
        Tag::new(Class::Universal(Type::OctetString), Payload::Primitive(self.into_bytes()))
    }

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let payload = try!(tag.into_payload().into_inner_primitive());
        Ok(try!(String::from_utf8(payload)))
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

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let byte = try!(tag.into_payload().into_inner_primitive())[0];
        Ok(byte == 0xFF)
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

    fn from_tag(tag: Tag) -> LDAPResult<Self>
    {
        let bytes = try!(tag.into_payload().into_inner_primitive());
        let mut res = 0i32;
        let mut i = 0u8;
        for byte in bytes
        {
            res = byte as i32 >> (32 - i * 8);
            i += 1;
        }
        Ok(res)
    }
}
