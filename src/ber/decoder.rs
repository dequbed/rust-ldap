use ber;
use ber::error::ASN1Error as Error;
use ber::common::{self, Tag};

use std::io;
use std::io::{Read, Take};

use byteorder::BigEndian;
use byteorder::ReadBytesExt;

pub struct Decoder<R>
{
    rdr: R,
}

impl<R: io::Read> Decoder<R>
{
    pub fn from_reader(rdr: R) -> Decoder<io::BufReader<R>>
    {
        Decoder::from_reader_raw(io::BufReader::new(rdr))
    }

    pub fn from_reader_raw(rdr: R) -> Decoder<R>
    {
        Decoder
        {
            rdr: rdr,
        }
    }

    pub fn decode(&mut self) -> ber::Result<common::Tag>
    {
        let _type = try!(read_type(&mut self.rdr));
        let _length = try!(read_length(&mut self.rdr));
        let _value = try!(read_value(_type.structure, io::Read::take(&mut self.rdr, _length as u64)));

        Ok(Tag
        {
            size: common::calculate_len(&_type, &_length),
            _type: _type,
            _length: _length,
            _value: _value,
        })
    }
}

// Decode a stream of bytes into an assortment of tags

pub fn read(r: &mut Read) -> ber::Result<Tag>
{
    let _type = try!(read_type(r));
    let _length = try!(read_length(r));
    let _value = try!(read_value(_type.structure, r.take(_length as u64)));

    Ok(Tag
    {
        size: common::calculate_len(&_type, &_length),
        _type: _type,
        _length: _length,
        _value: _value,
    })
}

fn read_type(r: &mut Read) -> ber::Result<common::Type>
{
    let first_byte = try!(r.read_u8());

    let class = first_byte >> 6;
    let structure = common::Structure::from_u8((first_byte & 0x20) >> 5);
    let number = first_byte & 0x1F;

    // Tags are using the extended form
    if number == 0x1F
    {
        let mut tag = 0u32;

        for count in 0..
        {
            let byte = try!(r.read_u8());

            // The first bit does not count towards the final ID
            let nbr = (byte & 0x7F) as u32;

            tag = tag << 7;
            tag += nbr;

            // If the 8th bit is not set this byte is the last
            if byte & 0x80 == 0
            {
                break;
            }
        }

        let class = try!(common::Class::construct(class, tag as i64));
        Ok(common::Type {
            class: class,
            structure: structure
        })
    }
    else
    {
        let class = try!(common::Class::construct(class, number as i64));
        Ok(common::Type {
            class: class,
            structure: structure
        })
    }
}

fn read_length(r: &mut Read) -> ber::Result<u64>
{
    let first_byte = try!(r.read_u8());

    if first_byte == 0x80
    {
        // Indefinite lenght. Not valid in LDAP
        return Err(Error::IndefiniteLength);
    }

    // First bit is set. Either we're using indefinite length or the long form
    if first_byte > 0x80
    {
        return Ok(try!(r.read_uint::<BigEndian>((first_byte & 0x7f) as usize)) as u64);
    }

    // Using the short form
    Ok(first_byte as u64)
}

fn read_value(s: common::Structure, mut t: Take<&mut Read>) -> ber::Result<common::Payload>
{
    match s
    {
        common::Structure::Primitive =>
        {
            let mut buffer: Vec<u8> = Vec::with_capacity(t.limit() as usize);
            try!(t.read_to_end(&mut buffer));
            Ok(common::Payload::Primitive(buffer))
        },
        common::Structure::Constructed =>
        {
            // Parse
            let mut tags = Vec::<common::Tag>::new();

            // Constructed tags may be empty
            let length = t.limit();
            if length > 0
            {
                let mut left = length;
                while
                {
                    let tag = try!(read(&mut t));
                    let read_len = common::calculate_len(&tag._type, &tag._length);
                    tags.push(tag);
                    left -= read_len as u64;

                    println!("{}", left);

                    // If this returns false the while loop ends
                    left > 0
                } {}
            }

            Ok(common::Payload::Constructed(tags))
        },
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use ber::common;
    use std::io::Cursor;

    #[test]
    fn decode_primitive_tag()
    {
        let mut bytestream = Cursor::new(vec![2, 2, 255, 127]);
        let tag = super::read(&mut bytestream).unwrap();

        assert!(tag == common::Tag {
            _type: common::Type {
                    class: common::Class::Universal(common::UniversalTypes::Integer),
                    structure: common::Structure::Primitive
                },
            _length: 2,
            _value: common::Payload::Primitive(vec![255, 127]),
            size: 4,
        })
    }

    #[test]
    fn decode_constructed_tag()
    {
        let mut bytestream = Cursor::new(vec![48,14,12,12,72,101,108,108,111,32,87,111,114,108,100,33]);
        let tag = super::read(&mut bytestream).unwrap();

        assert!(tag == common::Tag {
            _type: common::Type {
                    class: common::Class::Universal(common::UniversalTypes::Sequence),
                    structure: common::Structure::Constructed,
                },
            _length: 14,
            _value: common::Payload::Constructed(vec![
                common::Tag {
                    _type: common::Type {
                            class: common::Class::Universal(common::UniversalTypes::Utf8String),
                            structure: common::Structure::Primitive,
                        },
                    _length: 12,
                    _value: common::Payload::Primitive("Hello World!".to_string().into_bytes()),
                    size: 14
                }]),
            size: 16,
        })
    }

    #[test]
    fn decode_extended_type_tags()
    {
        let mut bytestream = Cursor::new(vec![0x9F,0x87,0x68,0x06,0x73,0x65,0x63,0x6F,0x6E,0x64]);
        let tag = super::read(&mut bytestream).unwrap();

        println!("{:?}", tag);

        assert!(tag == common::Tag {
            _type: common::Type {
                    class: common::Class::ContextSpecific(1000),
                    structure: common::Structure::Primitive
                },
            _length: 6,
            _value: common::Payload::Primitive("second".to_string().into_bytes()),
            size: 10,
        });
    }

    #[test]
    fn decode_long_length_tags()
    {
        // I am so sorry D:
        let mut bytestream = Cursor::new(vec![
            0x30, 0x82, 0x01, 0x01,
            0x80, 0x0C, 0x4A, 0x75,
            0x73, 0x74, 0x41, 0x4C,
            0x6F, 0x6E, 0x67, 0x54,
            0x61, 0x67, 0x81, 0x81,
            0xF0, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67,
        ]);
        let tag = super::read(&mut bytestream).unwrap();

        assert!(tag == common::Tag {
            _type: common::Type {
                    class: common::Class::Universal(common::UniversalTypes::Sequence),
                    structure: common::Structure::Constructed,
                },
            _length: 257,
            _value: common::Payload::Constructed(vec![
                common::Tag {
                    _type: common::Type {
                            class: common::Class::ContextSpecific(0),
                            structure: common::Structure::Primitive,
                        },
                    _length: 12,
                    _value: common::Payload::Primitive("JustALongTag".to_string().into_bytes()),
                    size: 14
                },
                common::Tag {
                    _type: common::Type {
                            class: common::Class::ContextSpecific(1),
                            structure: common::Structure::Primitive,
                        },
                    _length: 240,
                    // Sorry D:
                    _value: common::Payload::Primitive("JustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTag".to_string().into_bytes()),
                    size: 243
                },
            ]),
            size: 261
        });
    }
}
