use std::collections::BTreeSet;

use crate::Error;
use crate::Result;

const MULTIPLIER: u64 = 6364136223846793005;
const MULTIPLIER_INV: u64 = 13877824140714322085;

const INCREMENT: u64 = 1;

/// Simplified LCG with 64-bit internal state and 32-bit outputs.
pub struct LCG {
    state: u64,
}

impl LCG {
    pub fn new(state: u64) -> Self {
        Self { state }
    }
    pub fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.next();
        (state >> 32) as u32
    }
    fn next(&mut self) {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
    }
}

pub fn crack_lcg(stream: &[u32]) -> Result<Vec<u64>> {
    if stream.len() < 2 {
        return Err(Error::InsufficientStream);
    }
    // Find l s.t. ((stream[0] << 32 | l) * MULTIPLIER) >> 32 == stream[1]
    let fst = (stream[0] as u64) << 32;
    let diff = ((stream[1] as u64) << 32).wrapping_sub(fst.wrapping_mul(MULTIPLIER));
    let ls = find_l((diff >> 32) as u32);
    let mut cand = vec![];
    for l in ls {
        let state = (stream[0] as u64) << 32 | l as u64;
        let mut lcg = LCG::new(state);
        // Check if the generated stream matches the given one
        let mut matches = true;
        for &value in stream {
            if lcg.next_u32() != value {
                matches = false;
                break;
            }
        }
        if matches {
            cand.push(state);
        }
    }
    Ok(cand)
}

fn find_l(diff: u32) -> Vec<u32> {
    let mut cand = Vec::new();
    let diff = diff as u64;
    // Baby-step giant-step
    // diff * 2^32 <= (big << 16 + small) * mult + increment < (diff + 1) * 2^32 is equivalent to
    // -small * mult <= big << 16 * mult - diff * 2^32 + increment < -small * mult + 2^32.
    // Therefore, first we store all the possibilities of big << 16 * mult - diff,
    // second we calculate the range [-small * mult, -small * mult + 2^32)
    // and check if any numbers above are contained in this interval.
    let mut giant = BTreeSet::new();
    for i in 0..1u64 << 16 {
        giant.insert((
            (i << 16)
                .wrapping_mul(MULTIPLIER)
                .wrapping_sub(diff << 32)
                .wrapping_add(INCREMENT),
            i << 16,
        ));
    }
    // TODO: handle cases where hi = 0
    for small in 0..1u64 << 16 {
        let lo = small.wrapping_neg().wrapping_mul(MULTIPLIER);
        let hi = lo.wrapping_add(1u64 << 32);
        for (value, big) in giant.range((lo, 0)..(hi, 0)) {
            cand.push((big | small) as u32);
        }
    }
    cand
}
