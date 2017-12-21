pub const GR_HEADER_MAGIC: u16 = 0x5FF0;
pub const GR_HEADER_VERSION: u8 =  0x01;

// PST Type identifiers 
pub const PST_TRUE: u8 = 0x0; 
pub const PST_FALSE: u8 = 0x1;
pub const PST_SYMBOL: u8 = 0x2;
pub const PST_INT32: u8 = 0x3; 
pub const PST_DOUBLE: u8 = 0x4; 
pub const PST_COMPLEX: u8 = 0x5; 
pub const PST_NULL: u8 = 0x6; 
pub const PST_PAIR: u8 = 0x7; 
pub const PST_VECTOR:u8  = 0x8; 
pub const PST_DICT: u8 = 0x9; // not implemented by gr_zeromq
pub const PST_UNIFORM_VECTOR: u8 = 0xa; 
pub const PST_UINT64: u8 = 0x0b; 
pub const PST_TUPLE: u8 = 0x0c;
pub const PST_COMMENT: u8 = 0x3b; // not implemented by gr_zeromq
pub const PST_COMMENT_END: u8 = 0x0a; // not implemented by gr_zeromq
pub const UVI_ENDIAN_MASK: u8 = 0x80; 
pub const UVI_SUBTYPE_MASK: u8 = 0x7f; 
pub const UVI_LITTLE_ENDIAN: u8 = 0x0; 
pub const UVI_BIG_ENDIAN: u8 = 0x80; 

// UVI type identifiers, (these are used to identify the type that a uniform vector holds)
pub const UVI_U8: u8 = 0x0; 
pub const UVI_S8: u8 = 0x1; 
pub const UVI_U16: u8 = 0x2; 
pub const UVI_S16: u8 = 0x3; 
pub const UVI_U32: u8 = 0x4; 
pub const UVI_S32: u8 = 0x5; 
pub const UVI_U64: u8 = 0x6; 
pub const UVI_S64: u8 = 0x7; 
pub const UVI_F32: u8 = 0x8; 
pub const UVI_F64: u8 = 0x9; 
pub const UVI_C32: u8 = 0xa; 
pub const UVI_C64: u8 = 0xb; 
