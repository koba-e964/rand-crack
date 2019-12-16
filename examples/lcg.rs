extern crate rand_crack;

use rand_crack::lcg::{crack_lcg, LCG};

fn main() {
    let state = 0x0011_4514;
    eprintln!("state={}", state);
    let mut lcg = LCG::new(state);
    let len = 2;
    let mut stream = vec![0; len];
    for item in &mut stream {
        *item = lcg.next_u32();
    }
    let recovered_state = crack_lcg(&stream).unwrap();
    eprintln!("recovered_state = {:?}", recovered_state);
}
