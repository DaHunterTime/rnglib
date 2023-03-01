mod values;
mod algorithm;
mod xorshift;
mod mersennetwister;
mod rand;

pub use values::{ValidRandomNumber, ValidRandomRange};
pub use algorithm::RandomAlgorithm;
pub use xorshift::{XORShift32, XORShift64, XORShift128, XORShift128Plus};
pub use mersennetwister::MersenneTwister;
pub use rand::Random;

#[macro_export]
macro_rules! random {
    () => {{
        let rng: random::Random<random::MersenneTwister> = random::Random::new();
        rng
    }};

    ($seed: expr) => {{
        let rng: random::Random<random::MersenneTwister> = match random::Random::seed($seed) {
            Ok(r) => r,
            Err(_) => random::Random::new()
        };

        rng
    }};
}
