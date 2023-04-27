use crate::values::{ValidRandomNumber, ValidRandomRange};

/// The `RandomAlgorithm` trait.
/// 
/// This trait defines what a struct needs to implement to be considered a valid random algorithm.
/// 
/// It defines a `Seed` type and a `Number` type, the latter needing to implement the
/// `ValidRandomNumber` trait.
pub trait RandomAlgorithm {
    type Seed;
    type Number: ValidRandomNumber;

    /// Creates a new algorithm with the given seed value.
    /// 
    /// Returns a `Result` due to the fact that some implementations may fail due to the use of
    /// invalid seeds.
    fn new(seed: Self::Seed) -> Result<Self, &'static str> where Self: Sized;

    /// Creates and returns a default implementation, generally with the time as a seed.
    fn default() -> Self;

    /// Returns a random number in the given range.
    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number;
}
