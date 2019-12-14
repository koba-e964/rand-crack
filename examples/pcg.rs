extern crate rand_core;
extern crate rand_crack;
extern crate rand_pcg;

use rand_core::RngCore;
use rand_pcg::Lcg64Xsh32;

use rand_crack::crack_pcg32;

fn main() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    let mut stream = [0; 4];
    for i in 0..4 {
        stream[i] = rng.next_u32();
        eprintln!("{}", stream[i]);
    }
    let cand = crack_pcg32(&stream);
    eprintln!("candidates = {:?}", cand);
}
