use std::time::{SystemTime, UNIX_EPOCH};

use crate::values::ValidRandomRange;
use crate::algorithm::RandomAlgorithm;

// Implementation for the Mersenne Twister
// https://en.wikipedia.org/wiki/Mersenne_Twister#Pseudocode
/// Mersenne Twister algorithm.
pub struct MersenneTwister {
    state: [u32; 624], // n = 624
    index: u32
}

impl MersenneTwister {
    // Coefficients:
    // w = 32 | n = 624        | m = 397 | r = 31         | a = 0x9908B0DF
    // u = 11 | d = 0xFFFFFFFF | s = 7   | b = 0x9D2C5680 |
    // t = 15 | c = 0xEFC60000 | l = 18  | f = 1812433253 |
    /// The twist operation, part of the algorithm.
    fn twist(&mut self) {
        let lower_mask = 0x7FFFFFFF; // (1 << 31) - 1;
        let upper_mask = 0x80000000;

        for i in 0..624 {
            // x = (state[i] & upper_mask) + (state[(i + 1) % n] & lower_mask)
            let x = (self.state[i] & upper_mask) + (self.state[(i + 1) % 624] & lower_mask);
            let mut y = x >> 1;

            if x % 2 != 0 {
                // y ^= a
                y ^= 0x9908B0DF;
            }

            // state[i] = state[(i + m) % n] ^ y
            self.state[i] = self.state[(i + 397) % 624] ^ y;
        }
    }
}

impl RandomAlgorithm for MersenneTwister {
    type Seed = u32;
    type Number = u32;

    // Coefficients:
    // w = 32 | n = 624        | m = 397 | r = 31         | a = 0x9908B0DF
    // u = 11 | d = 0xFFFFFFFF | s = 7   | b = 0x9D2C5680 |
    // t = 15 | c = 0xEFC60000 | l = 18  | f = 1812433253 |
    fn new(seed: Self::Seed) -> Result<MersenneTwister, &'static str> {
        // state = [0; n]
        let mut state: [u32; 624] = [0; 624];
        // index = n + 1
        let index = 625;

        state[0] = seed;

        for i in 1..624 {
            let idx = i as usize;
            // tmp = f * (state[i - 1] ^ (state[i - 1] >> w - 2)) + i
            let tmp = 1812433253_u32.wrapping_mul(state[idx - 1] ^ (state[idx - 1] >> 30)) + i;
            // state[i] = tmp & d
            state[idx] = tmp & 0xFFFFFFFF;
        }

        return Ok(MersenneTwister { state, index });
    }

    fn default() -> MersenneTwister {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.subsec_micros(),
            Err(_) => 1
        };

        return MersenneTwister::new(seed).unwrap();
    }

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number {
        // if index >= n
        if self.index >= 624 {
            self.twist();
            self.index = 0;
        }

        let mut x = self.state[self.index as usize];
        // x ^= (x >> u) & d
        x ^= (x >> 11) & 0xFFFFFFFF;
        // x ^= (x << s) & b
        x ^= (x << 7) & 0x9D2C5680;
        // x ^= (x << t) & c
        x ^= (x << 15) & 0xEFC60000;
        x ^= x >> 1;

        self.index += 1;

        return (x & 0xFFFFFFFF) % (range._end() - range._start()) + range._start();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mersenne_twister_random_value() {
        let mut random = MersenneTwister::new(10).unwrap();
        let value = random.randrange(1..5);
        assert_eq!(value, 3);
    }
}
