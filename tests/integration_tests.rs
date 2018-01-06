extern crate zmq;
extern crate cero;
use cero::deserialize;
use zmq::*;


#[cfg(test)]
#[test]
fn deserialize_known_good_gr_zmq_msg() {
    let dump: &[u8] = include_bytes!("dump.hex");
    let msg = deserialize(dump);
    println!("{:?}", msg);
    println!("{:?}", dump.len());
}

#[test]
#[ignore]
// Used for testing cero with a real gr_zmq connection
fn deserialize_zmq() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.connect("tcp://127.0.0.1:5555").unwrap();
    let subs = [];
    socket.set_subscribe(&subs);
    for _ in 0..20 {
        let zmq_msg = &socket.recv_bytes(0).unwrap();
        println!("{:?}", deserialize(zmq_msg));
    }
}