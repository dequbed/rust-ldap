use ber;
use ber::error::ASN1Error as Error;
use ber::common::{self, Tag};

use std::io;
use std::io::{Read, Take, Cursor};

use byteorder::BigEndian;
use byteorder::ReadBytesExt;

pub fn decode(buf: &[u8]) -> ber::Result<common::Tag>
{
    let buffer = buf.clone();
    let mut cursor = Cursor::new(buffer);
    let _type = try!(read_type(&mut cursor));
    let _length = try!(read_length(&mut cursor));

    let curpos = cursor.position() as usize;
    let endpos = curpos + _length as usize;
    let subslice = &cursor.get_mut()[curpos..endpos];
    let _value = try!(read_value(_type.structure, subslice));

    Ok(Tag
    {
        size: common::calculate_len(&_type, &_length),
        _type: _type,
        _length: _length,
        _value: _value,
    })
}

fn read_type(cursor: &mut Cursor<&[u8]>) -> ber::Result<common::Type>
{
    let first_byte = try!(cursor.read_u8());

    let class = first_byte >> 6;
    let structure = common::Structure::from_u8((first_byte & 0x20) >> 5);
    let number = first_byte & 0x1F;

    // Tags are using the extended form
    if number == 0x1F
    {
        let mut tag = 0u32;

        for count in 0..
        {
            let byte = try!(cursor.read_u8());

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

fn read_length(cursor: &mut Cursor<&[u8]>) -> ber::Result<u64>
{
    let first_byte = try!(cursor.read_u8());

    if first_byte == 0x80
    {
        // Indefinite lenght. Not valid in LDAP
        return Err(Error::IndefiniteLength);
    }

    // First bit is set. Either we're using indefinite length or the long form
    if first_byte > 0x80
    {
        return Ok(try!(cursor.read_uint::<BigEndian>((first_byte & 0x7f) as usize)) as u64);
    }

    // Using the short form
    Ok(first_byte as u64)
}

fn read_value(s: common::Structure, buf: &[u8]) -> ber::Result<common::Payload>
{
    match s
    {
        common::Structure::Primitive =>
        {
            // to_vec() copies
            Ok(common::Payload::Primitive(buf.to_vec()))
        },
        common::Structure::Constructed =>
        {
            // Parse
            let mut tags = Vec::<common::Tag>::new();

            // Constructed tags may be empty
            if buf.len() > 0
            {
                let mut left = buf.len();
                while
                {
                    let tag = try!(decode(buf));
                    let read_len = common::calculate_len(&tag._type, &tag._length);
                    tags.push(tag);
                    left -= read_len as usize;

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
        let mut bytestream = [2, 2, 255, 127];
        let tag = super::decode(&mut bytestream).unwrap();

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
        let mut bytestream = [48,14,12,12,72,101,108,108,111,32,87,111,114,108,100,33];
        let tag = super::decode(&mut bytestream).unwrap();

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

    // #[test]
    fn decode_extended_type_tags()
    {
        let mut bytestream = [0x9F,0x87,0x68,0x06,0x73,0x65,0x63,0x6F,0x6E,0x64];
        let tag = super::decode(&mut bytestream).unwrap();

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

    // #[test]
    fn decode_long_length_tags()
    {
        // I am so sorry D:
        let mut bytestream = [
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
        ];
        let tag = super::decode(&mut bytestream).unwrap();

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
