use crate::values::{ValidRandomNumber, ValidRandomRange};

pub trait RandomAlgorithm {
    type Seed;
    type Number: ValidRandomNumber;

    fn new(seed: Self::Seed) -> Result<Self, &'static str> where Self: Sized;

    fn default() -> Self;

    fn randrange<R: ValidRandomRange<Self::Number>>(&mut self, range: R) -> Self::Number;
}
