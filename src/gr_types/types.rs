use std::ffi::OsString;
// Mapping of GNURadio's PST_Types to our types 
#[derive(Debug)]
pub enum TagField {
    Bool(bool),
    Symbol(OsString), // I haven't looked into if we can use Rust strings here yet, so we default to OsString
    SymbolUTF8(String),
    Int32(i32), 
    Double(f64),
    Complex(f64, f64), // (real, imaginary)
    Null(),
    Pair(Box<TagField>, Box<TagField>), // GNURadio implements Pairs as recursivly so we will too
    Vector(Vec<TagField>), 
    Dict(), // Unimplemented
    UniformVector(Vec<UVIType>), 
    UInt64(u64),
    Tuple(Vec<TagField>), // We're choosing to represent tuples as vectors for now, we'll consider the performance implications later 
    // Used for errors
    Unimplemented(String),
    Unknown(String),
}

// UVI types, these only appear in Uniform Vectors
#[derive(Debug)]
pub enum UVIType {
    U8(u8),
    S8(i8),
    U16(u16),
    S16(i16),
    U32(u32), 
    S32(i32), 
    U64(u64), 
    S64(i64), 
    F32(f32), 
    F64(f64), 
    C32(f32, f32), 
    C64(f64, f64),
}