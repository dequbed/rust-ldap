use ber;
use ber::error::ASN1Error as Error;

use std::io::Write;

use byteorder::BigEndian;
use byteorder::WriteBytesExt;

use ber::common::{self, Tag};

pub fn write(tag: common::Tag, mut w: &mut Write) -> ber::Result<()>
{
    try!(write_type(tag._type, w));
    try!(write_length(tag._length, w));
    try!(write_value(tag._value, w));

    Ok(())
}

fn write_type(tagtype: common::Type, mut w: &mut Write) -> ber::Result<()>
{
    let class_number = match tagtype.class
    {
        common::Class::Universal(_) => common::ClassNumber::Universal,
        common::Class::Application(_) => common::ClassNumber::Application,
        common::Class::ContextSpecific(_) => common::ClassNumber::ContextSpecific,
        common::Class::Private(_) => common::ClassNumber::Private,
    };

    let mut extended_tag: Option<Vec<u8>> = None;

    let type_byte =
            // First two bits: Class
            (class_number as u8) << 6 |
            // Bit 6: Primitive/Constructed
            (tagtype.structure as u8) << 5 |
            // Bit 5-1: Tag Number
            match tagtype.class
            {
                // tag will never be bigger than 30 so this is ok
                common::Class::Universal(tag) => (tag as u8),
                common::Class::Application(tag) |
                common::Class::ContextSpecific(tag) |
                common::Class::Private(tag) =>
                {
                    if tag > 30
                    {
                        let mut tagbytes: Vec<u8> = Vec::new();

                        let mut tag = tag;
                        while tag > 0
                        {
                            let mut byte = (tag & 0x7F) as u8;

                            tag >>= 7;

                            tagbytes.push(byte);
                        }

                        extended_tag = Some(tagbytes);

                        // This means we need to set the 5 tag bits to 11111, so 31 or 0x1F
                        31
                    }
                    else
                    {
                        extended_tag = None;
                        (tag as u8)
                    }
                },
            }; // let type_byte

    try!(w.write_u8(type_byte));

    let mut written = 1;

    if let Some(mut ext_bytes) = extended_tag
    {
        for _ in 0..ext_bytes.len()-1
        {
            let mut byte = ext_bytes.pop().unwrap();

            // Set the first bit
            byte |= 0x80;

            w.write_u8(byte);
        }

        let byte = ext_bytes.pop().unwrap();
        w.write_u8(byte);
    }

    Ok(())
}

// Yes I know you could overflow the length in theory. But, do you have 2^64 bytes of memory?
fn write_length(mut length: u64, mut w: &mut Write) -> ber::Result<()>
{
    // Short form
    if length < 128
    {
        try!(w.write_u8(length as u8));

        Ok(())
    }
    // Long form
    else
    {
        let mut count = 0u8;
        let mut len = length;
        while {count += 1; len >>= 8; len > 0 }{}


        try!(w.write_u8(count | 0x80));
        w.write_uint::<BigEndian>(length, count as usize);

        Ok(())
    }
}

fn write_value(payload: common::Payload, mut w: &mut Write) -> ber::Result<()>
{
    match payload
    {
        common::Payload::Primitive(ref value) =>
        {
            try!(w.write_all(value));
            Ok(())
        },
        common::Payload::Constructed(tags) =>
        {
            for tag in tags
            {
                try!(write(tag, w));
            }
            Ok(())
        }
    }
}

// pub fn write(tag: Tag, mut w: &mut Write) -> io::Result<()>
// {
//     let class_number = match tag.class
//     {
//         Class::Universal(_) => ClassNumber::Universal,
//         Class::Application(_) => ClassNumber::Application,
//         Class::ContextSpecific(_) => ClassNumber::ContextSpecific,
//         Class::Private(_) => ClassNumber::Private,
//     };

//     let struct_number = match tag.payload
//     {
//         Payload::Primitive(_) => Structure::Primitive,
//         Payload::Constructed(_) => Structure::Constructed,
//     };

//     // Set to true if we have to write the tag in extended form
//     let mut extended_tag = false;

//     // Construct ASN1_Types byte
//     let type_byte =
//         // First two bits: Class
//         (class_number as u8) << 6 |
//         // Bit 6: Primitive/Constructed
//         (struct_number as u8) << 5 |
//         // Bit 5-1: Tag Number
//         match tag.class
//         {
//             // tag will never be bigger than 30 so this is ok
//             Class::Universal(tag) => (tag as u8),
//             Class::Application(tag) | Class::ContextSpecific(tag) | Class::Private(tag) =>
//             {
//                 if tag > 30
//                 {
//                     // Write tags in extended form
//                     extended_tag = true;
//                     // This means we need to set the 5 tag bits to 11111, so 31 or 0x1F
//                     31
//                 }
//                 else
//                 {
//                     (tag as u8)
//                 }
//             },
//         }; // let type_byte

//     // Write type byte
//     try!(w.write_u8(type_byte));

//     // Write the extended tag form
//     if extended_tag
//     {
//         match tag.class
//         {
//             Class::Universal(_) => unreachable!(),
//             Class::Application(tag) | Class::ContextSpecific(tag) | Class::Private(tag) =>
//             {
//                 let mut tag = tag;
//                 while tag > 0
//                 {

//                     let mut byte = (tag & 0x7F) as u8;

//                     // Shift away the 7 bits we just took
//                     tag >>= 7;
//                     if tag != 0
//                     {
//                         // There are more bytes to go, so set the 8th bit
//                         byte |= 0x80;
//                     }

//                     try!(w.write_u8(byte));
//                 }
//             },
//         }
//     }

//     // Write length
//     if tag.length < 0x80
//     {
//         // Use the short form
//         try!(w.write_u8(tag.length as u8));
//     }
//     else
//     {
//         // Long form has to be used
//         let mut count: u8 = 0;
//         let mut len = tag.payload.len();
//         // For each byte of length increase the count by one
//         while {count += 1; len >>= 8; len > 0 }{}

//         let count = count;
//         // Write the amount of length bytes that will follow
//         try!(w.write_u8(count));

//         for i in (0..count).rev()
//         {
//             // Write length bytes, most significant to least
//             // FIXME: This assumes little-endianess on the CPU
//             let byte = (
//                 // Zero out everything except the byte we care about and then shift
//                 // so only one byte is left
//                 (tag.payload.len() & (0xFF << i * 8)) >> i * 8
//                 ) as u8;

//             // Write the length bytes sequentially
//             try!(w.write_u8(byte))
//         }
//     }

//     // Finally, write the payload
//     match tag.payload
//     {
//         Payload::Primitive(ref value) =>
//         {
//             try!(w.write_all(value))
//         },
//         Payload::Constructed(ref tags) =>
//         {
//             // Recurse into each tag and let it write ittag
//             for tag in tags
//             {
//                 try!(tag.write(w));
//             }
//         },
//     }

//     // Everything worked :D
//     Ok(())
// } // write

#[cfg(test)]
mod test
{
    use super::*;
    use ber::common;
    use std::io::Cursor;

    use byteorder::WriteBytesExt;
    use byteorder::BigEndian;

    #[test]
    fn encode_simple_tag()
    {
        let mut payload: Vec<u8> = Vec::new();
        payload.write_i16::<BigEndian>(1616);

        let tagtype = common::Type {
            class: common::Class::Universal(common::UniversalTypes::Integer),
            structure: common::Structure::Primitive
        };
        let pl = common::Payload::Primitive(payload);

        let tag = common::construct(tagtype, pl);

        let mut buf = Vec::<u8>::new();
        super::write(tag, &mut buf).unwrap();

        println!("{:?}", buf);

        assert!(buf == vec![0x2, 0x2, 0x06, 0x50]);
    }

    #[test]
    fn encode_constructed_tag()
    {
        let child = {
            let tagtype = common::Type {
                class: common::Class::ContextSpecific(0),
                structure: common::Structure::Primitive
            };

            let pl = common::Payload::Primitive("Hello World!".to_string().into_bytes());

            common::construct(tagtype, pl)
        };

        println!("{:?}", child);

        let parent = {
            let tagtype = common::Type {
                class: common::Class::Universal(common::UniversalTypes::Sequence),
                structure: common::Structure::Constructed
            };

            let pl = common::Payload::Constructed(vec![child]);

            common::construct(tagtype, pl)
        };

        println!("{:?}", parent);

        let mut buf = Vec::<u8>::new();
        super::write(parent, &mut buf);

        println!("{:?}", buf);

        assert!(buf == vec![48, 14, 128, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33])
    }

    #[test]
    fn encode_extended_type_tags()
    {

        let tag = {
            let tagtype = common::Type {
                class: common::Class::ContextSpecific(1000),
                structure: common::Structure::Primitive
            };

            let pl = common::Payload::Primitive("second".to_string().into_bytes());

            common::construct(tagtype, pl)
        };

        let mut buf = Vec::<u8>::new();
        super::write(tag, &mut buf);

        println!("{:?}", buf);

        assert!(buf == vec![0x9F,0x87,0x68,0x06,0x73,0x65,0x63,0x6F,0x6E,0x64]);
    }

    #[test]
    fn encode_long_length_tags()
    {
        let name = {
            let tagtype = common::Type {
                class: common::Class::ContextSpecific(0),
                structure: common::Structure::Primitive,
            };

            let pl = common::Payload::Primitive("JustALongTag".to_string().into_bytes());

            common::construct(tagtype, pl)
        };

        let value = {
            let tagtype = common::Type {
                class: common::Class::ContextSpecific(1),
                structure: common::Structure::Primitive,
            };

            let pl = common::Payload::Primitive("JustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTagJustALongTag".to_string().into_bytes());

            common::construct(tagtype, pl)
        };

        let seq = {
            let tagtype = common::Type {
                class: common::Class::Universal(common::UniversalTypes::Sequence),
                structure: common::Structure::Constructed,
            };

            let pl = common::Payload::Constructed(vec![name, value]);

            common::construct(tagtype, pl)
        };

        let mut buf = Vec::<u8>::new();
        super::write(seq, &mut buf);

        println!("{:?}", buf);
        assert!(buf == vec![
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
    }
}
