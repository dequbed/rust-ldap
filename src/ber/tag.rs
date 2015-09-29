use err;

use std::io::{self, Write, Read};

use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Enum containing all Types Numbers
pub enum Type
{
    Eoc              = 0,
    Boolean          = 1,
    Integer          = 2,
    BitString        = 3,
    OctetString      = 4,
    Null             = 5,
    ObjectIdentifier = 6,
    ObjectDescriptor = 7,
    External         = 8,
    Real             = 9,
    Enumerated       = 10,
    EmbeddedPdv      = 11,
    Utf8String       = 12,
    RelativeOid      = 13,
    Sequence         = 16,
    Set              = 17,
    NumericString    = 18,
    PrintableString  = 19,
    T61String        = 20,
    VideotexString   = 21,
    Ia5String        = 22,
    UtcTime          = 23,
    GeneralizedTime  = 24,
    GraphicString    = 25,
    VisibleString    = 26,
    GeneralString    = 27,
    UniversalString  = 28,
    CharacterString  = 29,
    BmpString        = 30,
}

impl Type
{
    pub fn from_u8(v: u8) -> Type
    {
        match v
        {
            0 =>  Type::Eoc,
            1 =>  Type::Boolean,
            2 =>  Type::Integer,
            3 =>  Type::BitString,
            4 =>  Type::OctetString,
            5 =>  Type::Null,
            6 =>  Type::ObjectIdentifier,
            7 =>  Type::ObjectDescriptor,
            8 =>  Type::External,
            9 =>  Type::Real,
            10 => Type::Enumerated,
            11 => Type::EmbeddedPdv,
            12 => Type::Utf8String,
            13 => Type::RelativeOid,
            16 => Type::Sequence,
            17 => Type::Set,
            18 => Type::NumericString,
            19 => Type::PrintableString,
            20 => Type::T61String,
            21 => Type::VideotexString,
            22 => Type::Ia5String,
            23 => Type::UtcTime,
            24 => Type::GeneralizedTime,
            25 => Type::GraphicString,
            26 => Type::VisibleString,
            27 => Type::GeneralString,
            28 => Type::UniversalString,
            29 => Type::CharacterString,
            30 => Type::BmpString,
            // BER uses 5 bits to encode the universal tags, and 31/0x1F/b11111 is used
            // to signal to use the long form of encoding
            // FIXME: This is a public function, better error handling!
            _  => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Class
{
    // TODO: Use BigInt instead of i64 to make encoding arbitrary sizes possbile?
    // TODO: Find out if even necessary for LDAP
    Universal(Type),
    Application(i64),
    ContextSpecific(i64),
    Private(i64),
}

/// ClassNumber contains the numerical representation of the tags class
#[derive(Debug, Copy, Clone)]
enum ClassNumber
{
    Universal       = 0,
    Application     = 1,
    ContextSpecific = 2,
    Private         = 3,
}

impl ClassNumber
{
    pub fn from_u8(v: u8) -> ClassNumber
    {
        match v
        {
            0 => ClassNumber::Universal,
            1 => ClassNumber::Application,
            2 => ClassNumber::ContextSpecific,
            3 => ClassNumber::Private,
            // FIXME: Better Error handling
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Structure
{
    Primitive   = 0,
    Constructed = 1,
}

impl Structure
{
    pub fn from_u8(v: u8) -> Structure
    {
        match v
        {
            0 => Structure::Primitive,
            1 => Structure::Constructed,
            // FIXME: Better Error handling
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Payload
{
    Primitive(Vec<u8>),
    Constructed(Vec<Tag>),
}

impl Payload
{
    /// Return the length of the PAYLOAD in bytes
    ///
    /// If the payload is Constructed, this function
    /// returns the length of all contained tags, not
    /// just their payload.
    pub fn len(&self) -> usize
    {
        let mut l: usize = 0;
        match *self
        {
            Payload::Primitive(ref v) => l = v.len() as usize,
            Payload::Constructed(ref v) =>
            {
                for tag in v
                {
                    l += tag.len();
                }
            }
        }

        return l;

    }

    pub fn into_inner_constructed(self) -> Option<Vec<Tag>>
    {
        match self
        {
            Payload::Primitive(_) => None,
            Payload::Constructed(tags) => Some(tags),
        }
    }

    pub fn into_inner_primitive(self) -> Option<Vec<u8>>
    {
        match self
        {
            Payload::Constructed(_) => None,
            Payload::Primitive(vec) => Some(vec),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Tag
{
    pub class: Class,
    payload: Payload,
    // Length as encoded in the Tag -> Payload length, NOT TAG LENGTH
    length: u64,
}

impl Tag
{
    pub fn new(class: Class, payload: Payload) -> Tag
    {
        Tag
        {
            class: class,
            length: payload.len() as u64,
            payload: payload,
        }
    }
    pub fn read(r: &mut Read) -> Result<Tag, err::Error>
    {
        // First, read the type byte
        let tagbyte = try!(r.read_u8());

        // 0xC0 is 11000000 in binary
        let classnumber = ClassNumber::from_u8((tagbyte & 0xC0) >> 6);
        let structure = Structure::from_u8((tagbyte & 0x20) >> 5);

        let class: Class;

        if tagbyte & 0x1F == 0x1F
        {
            // Extended Tags
            class = match classnumber
            {
                ClassNumber::Universal => return Err(err::Error::new(err::Kind::InvalidLengthEncoding, None)),
                ClassNumber::Application => Class::Application(try!(Tag::read_extended_tags(r))),
                ClassNumber::ContextSpecific => Class::ContextSpecific(try!(Tag::read_extended_tags(r))),
                ClassNumber::Private => Class::Private(try!(Tag::read_extended_tags(r))),
            }
        }
        else
        {
            class = match classnumber
            {
                ClassNumber::Universal => Class::Universal(Type::from_u8(tagbyte & 0x1F)),
                ClassNumber::Application => Class::Application((tagbyte & 0x1F) as i64),
                ClassNumber::ContextSpecific => Class::ContextSpecific((tagbyte & 0x1F) as i64),
                ClassNumber::Private => Class::Private((tagbyte & 0x1F) as i64),
            };
        }

        let length = try!(Tag::read_lenght(r));

        let payload: Payload;

        // At last, get the payload
        match structure
        {
            Structure::Primitive =>
            {
                let mut buffer = vec![0; length as usize];
                try!(r.read(&mut buffer));
                payload = Payload::Primitive(buffer);
            },
            Structure::Constructed =>
            {
                // Parse
                let mut tags = Vec::new();

                let mut left = length;

                while
                {
                    let tag = try!(Tag::read(r));
                    left -= tag.len() as u64;
                    tags.push(tag);

                    // If this returns false the while loop ends
                    left > 0
                } {}

                payload = Payload::Constructed(tags);
            },
        }


        Ok(Tag
           {
               class: class,
               payload: payload,
               length: length,
           })
    }

    fn read_extended_tags(mut r: &mut Read) -> Result<i64, err::Error>
    {
        let mut count = 0usize;
        let mut tag = 0i64;

        while count < 8 // i64, so max 8 iterations have space
        {
            let byte = try!(r.read_u8());

            // The first bit does not count towards the final ID
            let nbr = (byte & 0x7F) as i64;

            tag |= nbr << (7 * count);

            // If the 8th bit is not set this byte is the last
            if byte & 0x80 == 0
            {
                break;
            }
            count += 1;
        }

        Ok(tag)
    }

    pub fn read_lenght(r: &mut Read) -> Result<u64, err::Error>
    {
        let lengthbyte = try!(r.read_u8());
        let mut length: u64 = 0;

        if lengthbyte == 0x80
        {
            // Indefinite length. NOPE. NOPENOPENOPE.
            return Err(err::Error::new(err::Kind::IndefiniteLength, None));
        }
        else if lengthbyte & 0x80 == 0x80
        {
            // Long form
            let count = (lengthbyte & 0x7F) as usize;

            for i in 0..count
            {
                let lengthbyte = try!(r.read_u8());
                length |= (lengthbyte as u64) << (count - i - 1 * 8);
            }
        }
        else
        {
            // Short form
            length = lengthbyte as u64;
        }

        Ok(length)
    }

    pub fn write(&self, mut w: &mut Write) -> io::Result<()>
    {
        let class_number = match self.class
        {
            Class::Universal(_) => ClassNumber::Universal,
            Class::Application(_) => ClassNumber::Application,
            Class::ContextSpecific(_) => ClassNumber::ContextSpecific,
            Class::Private(_) => ClassNumber::Private,
        };

        let struct_number = match self.payload
        {
            Payload::Primitive(_) => Structure::Primitive,
            Payload::Constructed(_) => Structure::Constructed,
        };

        // Set to true if we have to write the tag in extended form
        let mut extended_tag = false;

        // Construct Type byte
        let type_byte =
            // First two bits: Class
            (class_number as u8) << 6 |
            // Bit 6: Primitive/Constructed
            (struct_number as u8) << 5 |
            // Bit 5-1: Tag Number
            match self.class
            {
                // tag will never be bigger than 30 so this is ok
                Class::Universal(tag) => (tag as u8),
                Class::Application(tag) | Class::ContextSpecific(tag) | Class::Private(tag) =>
                {
                    if tag > 30
                    {
                        // Write tags in extended form
                        extended_tag = true;
                        // This means we need to set the 5 tag bits to 11111, so 31 or 0x1F
                        31
                    }
                    else
                    {
                        (tag as u8)
                    }
                },
            }; // let type_byte

        // Write type byte
        try!(w.write_u8(type_byte));

        // Write the extended tag form
        if extended_tag
        {
            match self.class
            {
                Class::Universal(_) => unreachable!(),
                Class::Application(tag) | Class::ContextSpecific(tag) | Class::Private(tag) =>
                {
                    let mut tag = tag;
                    while tag > 0
                    {
                        let mut byte = (tag & 0x7F) as u8;

                        // Shift away the 7 bits we just took
                        tag >>= 7;
                        if tag != 0
                        {
                            // There are more bytes to go, so set the 8th bit
                            byte |= 0x80;
                        }

                        try!(w.write_u8(byte));
                    }
                },
            }
        }

        // Write length
        if self.length < 0x80
        {
            // Use the short form
            try!(w.write_u8(self.length as u8));
        }
        else
        {
            // Long form has to be used
            let mut count: u8 = 0;
            let mut len = self.payload.len();
            // For each byte of length increase the count by one
            while {count += 1; len >>= 8; len > 0 }{}

            let count = count;
            // Write the amount of length bytes that will follow
            try!(w.write_u8(count));

            for i in (0..count).rev()
            {
                // Write length bytes, most significant to least
                // FIXME: This assumes little-endianess on the CPU
                let byte = (
                    // Zero out everything except the byte we care about and then shift
                    // so only one byte is left
                    (self.payload.len() & (0xFF << i * 8)) >> i * 8
                ) as u8;

                // Write the length bytes sequentially
                try!(w.write_u8(byte))
            }
        }

        // Finally, write the payload
        match self.payload
        {
            Payload::Primitive(ref value) =>
            {
                try!(w.write_all(value))
            },
            Payload::Constructed(ref tags) =>
            {
                // Recurse into each tag and let it write itself
                for tag in tags
                {
                    try!(tag.write(w));
                }
            },
        }

        // Everything worked :D
        Ok(())
    } // write

    // FIXME: This function does not yet work!
    /// Returns the length of the whole tag in bytes
    pub fn len(&self) -> usize
    {
        let mut length: usize = 0;

        // Get the Lenght of the Class/PC/Type byte(s)
        length += match self.class
        {
            Class::Universal(_) => /* Universal is always exactly one byte */ 1,
            Class::Application(tag) | Class::ContextSpecific(tag) | Class::Private(tag) =>
            {
                // In case of the other three we actually have to look at their content
                let mut len = 1usize;
                let mut tag = tag;
                while tag < 0
                {
                    tag >>= 8;
                    len += 1;
                }
                len
            }
        };

        // Add space the length bytes take up
        if self.length < 127
        {
            // Short form was used -> Just one byte
            length += 1;
        }
        else
        {
            // Long form was used
            // Mask out first bit and add amount of length bytes that will follow
            length += (self.length & 0x80) as usize;
            // Add 1 for the first length byte
            length += 1;
        }

        // Add payload length
        length += self.payload.len();

        length
    }

    pub fn is_class(&self, class: Class) -> bool
    {
        self.class == class
    }

    // Consume tag to extract payload
    pub fn into_payload(self) -> Payload
    {
        self.payload
    }

    pub fn set_class(&mut self, class: Class)
    {
        self.class = class;
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_simple_tag()
    {
        let mut bytestream = Cursor::new(vec![2, 2, 255, 127]);
        let tag = Tag::read(&mut bytestream).unwrap();

        assert!(tag == Tag {
            class: Class::Universal(Type::Integer),
            payload: Payload::Primitive(vec![255, 127]),
            length: 2,
        });
    }

    #[test]
    fn write_simple_tag()
    {
        let payload = vec![255, 127];
        let tag = Tag
        {
            class: Class::Universal(Type::Integer),
            payload: Payload::Primitive(payload.clone()),
            length: 2,
        };

        let mut buf = Vec::<u8>::new();
        tag.write(&mut buf).unwrap();
        assert!(buf == vec![0x2, 0x2, 0xFF, 0x7F]);
    }

    #[test]
    fn check_primitive_tag_length()
    {
        let content = "Hello World!".to_string();
        let tag = Tag
        {
            class: Class::Universal(Type::Utf8String),
            length: content.len() as u64,
            payload: Payload::Primitive(content.into_bytes()),
        };

        let mut buf = Vec::<u8>::new();
        tag.write(&mut buf).unwrap();

        assert!(tag.len() == 14);
    }

    #[test]
    fn read_constructed_tag()
    {
        let mut bytestream = Cursor::new(vec![48, 14, 12, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
        let tag = Tag::read(&mut bytestream).unwrap();

        assert!(tag == Tag
            {
                class: Class::Universal(Type::Sequence),
                length: 14u64,
                payload: Payload::Constructed(vec![
                    Tag
                    {
                        class: Class::Universal(Type::Utf8String),
                        length: 12u64,
                        payload: Payload::Primitive("Hello World!".to_string().into_bytes()),
                    }
                ]),
            })
    }

    #[test]
    fn write_constructed_tag()
    {
        let content = "Hello World!".to_string();
        let child = Tag
        {
            class: Class::Universal(Type::Utf8String),
            length: content.len() as u64,
            payload: Payload::Primitive(content.into_bytes()),
        };
        let parent = Tag
        {
            class: Class::Universal(Type::Sequence),
            length: child.len() as u64,
            payload: Payload::Constructed(vec![child]),
        };

        let mut buf = Vec::<u8>::new();
        parent.write(&mut buf).unwrap();

        assert!(buf == vec![48, 14, 12, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33])
    }

    #[test]
    fn check_constructed_tag_length()
    {
        let content = "Hello World!".to_string();
        let child = Tag
        {
            class: Class::Universal(Type::Utf8String),
            length: content.len() as u64,
            payload: Payload::Primitive(content.into_bytes()),
        };
        let parent = Tag
        {
            class: Class::Universal(Type::Sequence),
            length: child.len() as u64,
            payload: Payload::Constructed(vec![child]),
        };

        assert!(parent.len() == 16);
    }
}
