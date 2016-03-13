use ber;
use ber::error::ASN1Error as Error;
use ber::common::{self, Tag};

use std::io::{Read, Take};

use byteorder::BigEndian;
use byteorder::ReadBytesExt;

// Decode a stream of bytes into an assortment of tags

fn read(r: &mut Read) -> ber::Result<Tag>
{
    let _type = try!(read_type(r));
    let _length = try!(read_length(r));
    let _value = try!(read_value(_type.1, r.take(_length)));

    Ok(Tag
    {
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

        for count in 0..3
        {
            let byte = try!(r.read_u8());

            // The first bit does not count towards the final ID
            let nbr = (byte & 0x7F) as u32;

            tag |= nbr << (7 * count);

            // If the 8th bit is not set this byte is the last
            if byte & 0x80 == 0
            {
                break;
            }
        }

        let class = try!(common::Class::construct(class, tag as i64));
        Ok((class, structure))
    }
    else
    {
        let class = try!(common::Class::construct(class, number as i64));
        Ok((class, structure))
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
        return Ok(try!(r.read_uint::<BigEndian>((first_byte & 0x7f) as usize)));
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
                    let read_len = tag_len(&tag);
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

fn tag_len(tag: &common::Tag) -> usize
    {
        let mut length: usize = 0;

        // Get the Lenght of the Class/PC/Type byte(s)
        length += match tag._type.0
        {
            common::Class::Universal(_) => /* Universal is always exactly one byte */ 1,
            common::Class::Application(tag) | common::Class::ContextSpecific(tag) | common::Class::Private(tag) =>
            {
                // In case of the other three we actually have to look at their content
                let mut len = 1usize;
                if tag > 127
                {
                    let mut tag = tag;
                    while (tag >> 7) > 0
                    {
                        tag >>= 7;
                        len += 1;
                    }
                }
                len
            }
        };

        // Add space the length bytes take up
        if tag._length <= 127
        {
            // Short form was used -> Just one byte
            length += 1;
        }
        else
        {
            let mut len = tag._length;
            while len > 0
            {
                len >>= 8;
                length += 1;
            }

            length += 1;
        }

        // Add payload length
        length += tag._length as usize;

        length
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
            _type: (common::Class::Universal(common::UniversalTypes::Integer), common::Structure::Primitive),
            _length: 2,
            _value: common::Payload::Primitive(vec![255, 127])
        })
    }

    #[test]
    fn decode_constructed_tag()
    {
        let mut bytestream = Cursor::new(vec![48,14,12,12,72,101,108,108,111,32,87,111,114,108,100,33]);
        let tag = super::read(&mut bytestream).unwrap();

        assert!(tag == common::Tag {
            _type: (common::Class::Universal(common::UniversalTypes::Sequence), common::Structure::Constructed),
            _length: 14,
            _value: common::Payload::Constructed(vec![
                common::Tag {
                    _type: (common::Class::Universal(common::UniversalTypes::Utf8String), common::Structure::Primitive),
                    _length: 12,
                    _value: common::Payload::Primitive("Hello World!".to_string().into_bytes())
                }])
        })
    }
}
