extern crate rand_core;
extern crate rand_pcg;

use rand_core::RngCore;
use rand_pcg::Pcg32;

pub mod lcg;

#[derive(Debug, Clone)]
pub enum Error {
    InsufficientStream,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Two values are not sufficient to guess the state of the rng.
/// To recover, we need at least three u32 outputs.
/// Currently, this function tries 2^32 possibilities, so it's very slow.
/// TODO: optimization
pub fn crack_pcg32(stream: &[u32]) -> Result<Vec<(u64, u64)>> {
    if stream.len() < 3 {
        return Err(Error::InsufficientStream);
    }

    let increment = 3; // TODO: find this value as well

    // candidates of initial states.
    let mut cand = Vec::new();
    for rot in 0..32 {
        for lower in 0..1 << 27 {
            let state = recover_original_state(stream[0], rot, lower);
            let state = state
                .wrapping_sub(increment)
                .wrapping_mul(MULTIPLIER_INV)
                .wrapping_sub(increment);
            let mut rng = Pcg32::new(state, increment >> 1);
            assert_eq!(rng.next_u32(), stream[0]);
            if rng.next_u32() != stream[1] {
                continue;
            }
            if rng.next_u32() != stream[2] {
                continue;
            }
            cand.push((state, increment >> 1));
        }
    }
    Ok(cand)
}

fn recover_original_state(value: u32, rot: u32, lower: u32) -> u64 {
    debug_assert!(rot < 32);
    debug_assert!(lower < 1 << 27);
    let xsh = value.rotate_left(rot);
    let state: u64 = (rot as u64) << 32 | xsh as u64;
    let state = state ^ state >> 18;
    let state = state ^ state >> 36;
    let state = state << 27 | lower as u64;

    if cfg!(debug_assertions) {
        verify_original_state(value, rot, state);
    }
    state
}

#[allow(unused)]
const MULTIPLIER: u64 = 6_364_136_223_846_793_005; // used by tests
const MULTIPLIER_INV: u64 = 13_877_824_140_714_322_085;

fn verify_original_state(value: u32, rot: u32, state: u64) {
    // Excerpt from the original code.
    assert_eq!((state >> 59) as u32, rot);
    let xsh = (((state >> 18) ^ state) >> 27) as u32;
    assert_eq!(xsh.rotate_right(rot), value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier_inv() {
        assert_eq!(MULTIPLIER.wrapping_mul(MULTIPLIER_INV), 1);
    }

    #[test]
    fn recover_original_state_works() {
        let state = recover_original_state(127, 14, 0x1553a);
        verify_original_state(127, 14, state);
        let state = recover_original_state(0x03a2_b112, 14, 0x1553a);
        verify_original_state(0x03a2_b112, 14, state);
    }
}
