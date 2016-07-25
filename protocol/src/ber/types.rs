use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ber::{common};

pub trait ASNType
{
    // Encode as BER Tag using Universal types.
    fn into_ber_universal(self) -> common::Tag;

    // Encode as BER Tag with the given class.
    fn into_ber_typed(self, class: common::Class) -> common::Tag;

    // Try to decode Tag into Type.
    fn from_tag(tag: &mut common::Tag) -> Option<Self> where Self: Sized;
}

impl ASNType for i32
{
    fn into_ber_universal(self) -> common::Tag
    {
        self.into_ber_typed(common::Class::Universal(common::UniversalTypes::Integer))
    }

    fn into_ber_typed(self, class: common::Class) -> common::Tag
    {
        // It's a u32, we know it's always 4 bytes
        let mut payload: Vec<u8> = Vec::with_capacity(4);
        payload.write_i32::<BigEndian>(self);

        let pl = common::Payload::Primitive(payload);

        common::construct(class, pl)
    }

    fn from_tag(tag: &mut common::Tag) -> Option<Self>
    {
        match tag._value
        {
            common::Payload::Primitive(ref vec) =>
            {
                if vec.len() <= 4
                {
                    let mut rdr = Cursor::new(vec);
                    match rdr.read_int::<BigEndian>(vec.len()).ok()
                    {
                        None => None,
                        Some(i) => Some(i as i32),
                    }
                }
                else
                {
                    None
                }
            },
            common::Payload::Constructed(_) => None
        }
    }
}

impl ASNType for i64
{
    fn into_ber_universal(self) -> common::Tag
    {
        self.into_ber_typed(common::Class::Universal(common::UniversalTypes::Integer))
    }

    fn into_ber_typed(self, class: common::Class) -> common::Tag
    {
        // It's a u32, we know it's always 4 bytes
        let mut payload: Vec<u8> = Vec::with_capacity(8);
        payload.write_i64::<BigEndian>(self);

        let pl = common::Payload::Primitive(payload);

        common::construct(class, pl)
    }

    fn from_tag(tag: &mut common::Tag) -> Option<Self>
    {
        match tag._value
        {
            common::Payload::Primitive(ref vec) =>
            {
                if vec.len() <= 8
                {
                    let mut rdr = Cursor::new(vec);
                    rdr.read_int::<BigEndian>(vec.len()).ok()
                }
                else
                {
                    None
                }
            },
            common::Payload::Constructed(_) => None
        }
    }
}

impl ASNType for Vec<common::Tag>
{
    fn into_ber_universal(self) -> common::Tag
    {
        self.into_ber_typed(common::Class::Universal(common::UniversalTypes::Sequence))
    }

    fn into_ber_typed(self, class: common::Class) -> common::Tag
    {
        let pl = common::Payload::Constructed(self);
        common::construct(class, pl)
    }

    fn from_tag(tag: &mut common::Tag) -> Option<Self>
    {
        None
    }
}
