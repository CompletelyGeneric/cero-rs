#![feature(test)]
extern crate test;
extern crate cero;
extern crate zmq;
use zmq::{ Context };
use cero::deserialize;





#[bench]
fn deser_from_file(b : &mut test::Bencher) {
    let dump: &[u8] = include_bytes!("dump.hex");
    b.iter(|| { deserialize(dump) });
} 

#[bench]
fn deser_from_file_bb(b : &mut test::Bencher) {
    let dump: &[u8] = include_bytes!("dump.hex");
    b.iter(|| { test::black_box(deserialize(dump)) });
} 
#[bench]
fn deser_many_from_file(b : &mut test::Bencher) {
    let dump: &[u8]= include_bytes!("dump.hex");
    b.iter(|| { for _ in 0..100 { deserialize(dump); }});
} 

#[bench]
fn deser_many_from_file_bb(b : &mut test::Bencher) {
    let dump: &[u8] = include_bytes!("dump.hex");
    b.iter(|| { for _ in 0..100 { test::black_box(deserialize(dump)); }});
} 

#[bench]
fn deserialize_from_vec(b: &mut test::Bencher) {
    let dump: &[u8] = include_bytes!("dump.hex");
    b.iter(|| {  test::black_box(deserialize(Vec::from(dump))); });
}

#[bench]
#[ignore]
fn deserialize_zmq(b : &mut test::Bencher) {
    let ctx = zmq::Context::new();
    let mut socket = ctx.socket(zmq::SUB).unwrap();
    socket.connect("tcp://127.0.0.1:5555").unwrap();
    let subs = [];
    socket.set_subscribe(&subs);
    b.iter(|| {  
        test::black_box(deserialize(&socket.recv_bytes(0).unwrap())); 
        });     
}