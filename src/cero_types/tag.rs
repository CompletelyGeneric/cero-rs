use std::fmt;
use gr_types::types::TagField;

pub struct Tag {
    pub offset: u64,
    pub key: TagField,
    pub value: TagField,
}

impl Tag {
    pub fn new() -> Tag {
        return Self {offset: 0, key: TagField::Null(), value: TagField::Null()};
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TAG: Offset: {:?}, Key: {:?}, Value: {:?}", self.offset, self.key, self.value)
    }
}