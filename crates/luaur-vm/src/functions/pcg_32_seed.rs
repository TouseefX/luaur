use crate::functions::pcg_32_random::pcg_32_random;

pub fn pcg_32_seed(state: &mut u64, seed: u64) {
    *state = 0;
    pcg_32_random(state);
    *state += seed;
    pcg_32_random(state);
}
