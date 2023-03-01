use std::time::{SystemTime, UNIX_EPOCH};

use crate::values::ValidRandomRange;
use crate::algorithm::RandomAlgorithm;

// Implementation for linear xor shift algorithms
// https://en.wikipedia.org/wiki/Xorshift#Example_implementation
pub struct XORShift32 {
    state: u32
}

impl RandomAlgorithm for XORShift32 {
    type Seed = u32;
    type Number = u32;

    fn new(seed: Self::Seed) -> Result<XORShift32, &'static str> {
        if seed == 0 {
            return Err("seed must be initialized to non-zero");
        }

        return Ok(XORShift32 { state: seed });
    }

    fn default() -> XORShift32 {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.subsec_micros(),
            Err(_) => 1
        };

        return XORShift32 { state: seed };
    }

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;

        return x % (range._end() - range._start()) + range._start();
    }
}

pub struct XORShift64 {
    state: u64
}

impl RandomAlgorithm for XORShift64 {
    type Seed = u64;
    type Number = u64;

    fn new(seed: Self::Seed) -> Result<XORShift64, &'static str> {
        if seed == 0 {
            return Err("seed must be initialized to non-zero");
        }

        return Ok(XORShift64 { state: seed });
    }

    fn default() -> XORShift64 {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 1
        };

        return XORShift64 { state: seed };
    }

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;

        return x % (range._end() - range._start()) + range._start();
    }
}

pub struct XORShift128 {
    state: u128
}

impl RandomAlgorithm for XORShift128 {
    type Seed = u128;
    type Number = u128;

    fn new(seed: Self::Seed) -> Result<XORShift128, &'static str> {
        if seed == 0 {
            return Err("seed must be initialized to non-zero");
        }

        return Ok(XORShift128 { state: seed });
    }

    fn default() -> XORShift128 {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_micros(),
            Err(_) => 1
        };

        return XORShift128 { state: seed };
    }

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number {
        let mut x = self.state;
        x ^= x << 11;
        x ^= x >> 8;
        x ^= x << 19;
        self.state = x;

        return x % (range._end() - range._start()) + range._start();
    }
}

// Implementation for xor shift + algorithm
// https://en.wikipedia.org/wiki/Xorshift#xorshift+
pub struct XORShift128Plus {
    state: [u64; 2]
}

impl RandomAlgorithm for XORShift128Plus {
    type Seed = [u64; 2];
    type Number = u128;

    fn new(seed: Self::Seed) -> Result<XORShift128Plus, &'static str> {
        if seed[0] | seed[1] == 0 {
            return Err("at least one bit of the seed must be initialized to non-zero");
        }

        return Ok(XORShift128Plus { state: seed });
    }

    fn default() -> XORShift128Plus {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 1
        };

        return XORShift128Plus { state: [seed, seed + 1] };
    }

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number {
        let mut x = self.state[0];
        let y = self.state[1];
        x ^= x << 23;
        x ^= x >> 18;
        x ^= y ^ (y >> 5);
        self.state[1] = x;

        return u128::from(x + y) % (range._end() - range._start()) + range._start();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift32_random_value() {
        let mut random = XORShift32::new(10).unwrap();
        let value = random.randrange(1..5);
        assert_eq!(value, 3);
    }

    #[test]
    fn xorshift64_random_value() {
        let mut random = XORShift64::new(10).unwrap();
        let value = random.randrange(1..5);
        assert_eq!(value, 3);
    }

    #[test]
    fn xorshift128_random_value() {
        let mut random = XORShift128::new(10).unwrap();
        let value = random.randrange(1..5);
        assert_eq!(value, 3);
    }

    #[test]
    fn xorshift128plus_random_value() {
        let mut random = XORShift128Plus::new([10, 20]).unwrap();
        let value = random.randrange(1..5);
        assert_eq!(value, 3);
    }
}
