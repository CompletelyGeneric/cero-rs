use bytes::{Bytes, Buf, IntoBuf, BigEndian, LittleEndian};
// use bytes::buf::FromBuf;
use gr_types::magic::*;
use gr_types::types::{TagField, UVIType};
use cero_types::msg::{Msg, Header};
use cero_types::tag::Tag;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt; 



pub fn deserialize(gr_zmq_msg: &[u8]) -> Msg { 
    let mut buf = Bytes::from(gr_zmq_msg).into_buf();
    // Validate magic and version at some point
    let gr_magic = buf.get_u16::<LittleEndian>();
    let gr_version = buf.get_u8();

    let offset = buf.get_u64::<LittleEndian>();
    let num_tags = buf.get_u64::<LittleEndian>();
    let header = Header {offset: offset, num_tags: num_tags};
    let tags: Vec<Tag> = Vec::new();
    let mut msg = Msg {header: header, tags: tags};
    //let mut buf_ref = buf.by_ref();
    
    for _ in 0..num_tags {
        msg.tags.push(parse_tag(&mut buf))
    }
    return msg;
}


pub fn parse_tag<T: Buf>(buf: &mut T) -> Tag {
    let tag_offset = buf.get_u64::<LittleEndian>();
    let key = parse_tag_field(buf);
    let value = parse_tag_field(buf);
    let _ = parse_tag_field(buf); // Currently we throw away the src_id, we should probably just have it as an Option
    let tag = Tag {offset: tag_offset, key, value};
    return tag;
}


pub fn parse_tag_field<T: Buf>(buf: &mut T) -> TagField {
    let tag_type = buf.get_u8();
    let field: TagField = match tag_type {
        PST_TRUE => TagField::Bool(true),
        PST_FALSE => TagField::Bool(false),
        PST_SYMBOL => { 
            let len = buf.get_u16::<BigEndian>() as usize; 
            let symbol = OsStr::from_bytes(&buf.bytes()[0..len]).to_os_string(); // 0.2.0 will fix this hopefully
            buf.advance(len);
            TagField::Symbol(symbol)
        },
        PST_INT32 =>  TagField::Int32(buf.get_i32::<BigEndian>()),
        PST_UINT64 => TagField::UInt64(buf.get_u64::<BigEndian>()),
        PST_PAIR => {
            let mut left = Box::new(parse_tag_field(buf));
            let mut right = Box::new(parse_tag_field(buf));
            TagField::Pair(left, right)
        },
        PST_DOUBLE =>  TagField::Double(buf.get_f64::<BigEndian>()),
        PST_COMPLEX => TagField::Complex(buf.get_f64::<BigEndian>(), buf.get_f64::<BigEndian>()), // (real, imaginary)
        PST_TUPLE => { // We're chooseing to represent tuples as vectors for the time being
            let num_items = buf.get_u32::<BigEndian>();
            let mut vec = Vec::<TagField>::new();
            for _ in 0..num_items {
                vec.push(parse_tag_field(buf));
            }
            TagField::Tuple(vec)
        },
        PST_VECTOR => { // A PST_Vector can hold any type defined in our TagField enum
            let num_items = buf.get_i32::<BigEndian>(); 
            let mut vec = Vec::<TagField>::new();
            for _ in 0..num_items{
                vec.push(parse_tag_field(buf));
            }
            TagField::Vector(vec)
        },
        PST_UNIFORM_VECTOR => {
            let mut vec = Vec::<UVIType>::new();
            let uvi_type = buf.get_u8();
            let num_items = buf.get_i32::<BigEndian>();
            let pad = buf.get_u8(); 
            buf.advance(pad as usize); // throw away the padding
            match uvi_type { // Big ugly copy-paste type parsing logic, surely there's a better way
                UVI_U8 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::U8(buf.get_u8()));
                    }
                },
                UVI_S8 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::S8(buf.get_i8()));
                    }
                },
                UVI_U16 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::U16(buf.get_u16::<BigEndian>()));
                    }
                },
                UVI_S16 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::S16(buf.get_i16::<BigEndian>()));
                    }
                },
                UVI_U32 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::U32(buf.get_u32::<BigEndian>()));
                    }
                },
                UVI_S32 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::S32(buf.get_i32::<BigEndian>()));
                    }
                },
                UVI_U64 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::U64(buf.get_u64::<BigEndian>()));
                    }
                },
                UVI_S64 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::S64(buf.get_i64::<BigEndian>()));
                    }
                },
                UVI_F32 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::F32(buf.get_f32::<BigEndian>()));
                    }
                },
                UVI_F64 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::F64(buf.get_f64::<BigEndian>()));
                    }
                },
                UVI_C32 => { 
                    for _ in 0..num_items {
                        // GNURadio grabs these as f64's then casts them to f32's. Weird.
                        vec.push(UVIType::C32(buf.get_f64::<BigEndian>() as f32, buf.get_f64::<BigEndian>() as f32));
                    }
                },
                UVI_C64 => { 
                    for _ in 0..num_items {
                        vec.push(UVIType::C64(buf.get_f64::<BigEndian>(), buf.get_f64::<BigEndian>()));
                    }
                },
                _ => return TagField::Unknown(String::from(format!("ERROR: unknown tag type identifier \'{}\' ", tag_type))),
            };
            TagField::UniformVector(vec)
        }, 
        PST_COMMENT | PST_COMMENT_END => TagField::Unimplemented(String::from("ERROR: PST_COMMENT types not implemented")),
        PST_DICT => TagField::Unimplemented(String::from("ERROR: PST_DICT type not implemented")),
        _ => TagField::Unknown(String::from(format!("ERROR: unknown tag type identifier \'{}\' ", tag_type))),
    };
    return field;
}
