extern crate cero;
extern crate serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use cero::deserialize;

#[cfg(test)]
#[test]
fn serde_json() {
    let gr_dump: &[u8] = include_bytes!("dump.hex");
    let msg = deserialize(gr_dump);
    let json = serde_json::to_string_pretty(&msg).unwrap();
    let mut json_dump = File::create(Path::new("tests/dump.json")).expect("Couldn't open file");
    json_dump.write_all(json.as_bytes());
}