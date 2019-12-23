extern crate rand_core;
extern crate rand_crack;
extern crate rand_pcg;

use rand_core::RngCore;
use rand_pcg::Lcg64Xsh32;

use rand_crack::pcg32;

fn main() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    let mut stream = [0; 4];
    for item in &mut stream {
        *item = rng.next_u32();
        eprintln!("{}", item);
    }
    let cand = pcg32::crack(&stream);
    eprintln!("candidates = {:?}", cand);
}
