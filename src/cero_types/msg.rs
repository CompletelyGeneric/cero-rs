use cero_types::tag::Tag;
use std::fmt;
 
#[derive(Default)]
pub struct Header {
    pub offset: u64,
    pub num_tags: u64,
}

impl Header {
    pub fn new() -> Self {
        Self {offset: 0, num_tags: 0}
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offset: {:?}, Num' tags: {:?}", self.offset, self.num_tags)
    }
}

pub struct Msg {
    pub header: Header,
    pub tags: Vec<Tag>,
}

impl Msg {
    pub fn new() -> Self {
        Self { header: Header::new(), tags: Vec::new()}
    }
}


impl fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Header: {:?} \nTags: {:?}", self.header, self.tags)
    }
}

impl Default for Msg {
    fn default() -> Self {
        Self::new()
    }
}