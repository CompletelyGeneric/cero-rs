#![feature(test)]
extern crate test;
extern crate cero;
use cero::deserialize;





#[bench]
fn deser_from_file(b : &mut test::Bencher) {
    let dump = test::black_box(include_bytes!("dump.hex"));
    b.iter(|| { deserialize(dump) });
} 

#[bench]
fn deser_from_file_bb(b : &mut test::Bencher) {
    let dump = include_bytes!("dump.hex");
    b.iter(|| { test::black_box(deserialize(dump)) });
} 
#[bench]
fn deser_many_from_file(b : &mut test::Bencher) {
    let dump = include_bytes!("dump.hex");
    b.iter(|| { for _ in 0..100 { deserialize(dump); }});
} 

#[bench]
fn deser_many_from_file_bb(b : &mut test::Bencher) {
    let dump = include_bytes!("dump.hex");
    b.iter(|| { for _ in 0..100 { test::black_box(deserialize(dump)); }});
} 
// #[bench]
// fn deserialize_from_vec(b: &mut Bencher) {
//     let bytes = include_bytes!("dump.hex");
//     b.iter(|| { 
//         let bb = black_box(Vec::from(*bytes)); 
//         deserialize(bb) 
//         });
// }