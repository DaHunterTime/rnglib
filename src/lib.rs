//! # Pseudo-Random Number Generator Crate.
//! 
//! Provides pseudo-random number generation with multiple algorithms to choose from or to derive
//! your own.
//! 
//! # Quick Start
//! 
//! If you only want a random numbre and that's it then you can do something like the following.
//! 
//! ```rust
//! use rnglib:random;
//! 
//! fn main() {
//!   let mut rng = random!();
//!   let d20 = rng.randrange(1..=20); // get a random number in range [1, 20]
//!   println!("You rolled a {d20}");
//! }
//! ```
//! 
//! # Custom Random Algorithm
//! 
//! If you want to use your own random algorithm then you need to implement the `RandomAlgorithm`,
//! and depending on which values you want to get then you need to implement `ValidRandomNumber` for
//! the desired type.
//! 
//! # Warning
//! 
//! Do not use pseudo-random number generation for passwords or cryptographic needs.
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
/// The `random` macro can be used to create a default `Random` struct with the `MersenneTwister`
/// algorithm.
/// 
/// You can optionally provide a seed and it will try to create said struct with it.
macro_rules! random {
    () => {{
        let rng: rnglib::Random<rnglib::MersenneTwister> = rnglib::Random::new();
        rng
    }};

    ($seed: expr) => {{
        let rng: rnglib::Random<rnglib::MersenneTwister> = match rnglib::Random::seed($seed) {
            Ok(r) => r,
            Err(_) => rnglib::Random::new()
        };

        rng
    }};
}
