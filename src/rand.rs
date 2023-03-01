use std::collections::HashSet;

use crate::algorithm::RandomAlgorithm;
use crate::values::{ValidRandomNumber, ValidRandomRange};

pub struct Random<T>
    where T: RandomAlgorithm
{
    algorithm: T
}

impl<T> Random<T>
    where T: RandomAlgorithm
{
    pub fn new() -> Random<T> {
        return Random { algorithm: T::default() };
    }

    pub fn seed(seed: T::Seed) -> Result<Random<T>, &'static str> {
        return Ok(Random { algorithm: T::new(seed)? });
    }

    pub fn randrange<R: ValidRandomRange<T::Number>>(&mut self, range: R) -> T::Number {
        return self.algorithm.randrange(range);
    }

    pub fn random(&mut self) -> f64 {
        let value: T::Number = self.algorithm.randrange(T::Number::zero()..T::Number::max());
        return value.to_f64() / T::Number::max().to_f64();
    }

    pub fn uniform(&mut self, lower: T::Number, upper: T::Number) -> f64 {
        return lower.to_f64() + (upper - lower).to_f64() * self.random();
    }

    pub fn triangular(&mut self, lower: T::Number, upper: T::Number, mode: T::Number) -> f64 {
        let value = self.random();

        if value <= (mode - lower).to_f64() / (upper - lower).to_f64() {
            return (value * ((upper - lower).to_f64() * (mode - lower).to_f64())).sqrt()
                + lower.to_f64();
        }

        return upper.to_f64()
            - ((1.0 - value) * ((upper - lower).to_f64() * (upper - mode).to_f64())).sqrt();
    }

    pub fn randbytes(&mut self, amount: T::Number) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0; amount.to_usize()];

        for i in 0..amount.to_usize() {
            bytes[i] = self.randrange(T::Number::zero()..=T::Number::byte_max()).to_u8();
        }

        return bytes;
    }

    pub fn choose<'a, G>(&'a mut self, vector: &'a Vec<G>) -> &G {
        let end: T::Number = T::Number::from_usize(vector.len());
        let index = self.randrange(T::Number::zero()..end).to_usize();
        return &vector[index];
    }

    // The Fisher-Yates shuffle as described in
    // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
    pub fn shuffle<G>(&mut self, vector: &mut Vec<G>) {
        let mut items = vector.len() - 1;

        while items > 0 {
            let pos = self.randrange(
                T::Number::zero()..=T::Number::from_usize(items)
            ).to_usize();

            if pos != items {
                vector.swap(pos, items);
            }
    
            items -= 1;
        }
    }

    pub fn sample<'a, G>(
        &'a mut self, vector: &'a Vec<G>, amount: usize
    ) -> Result<Vec<&G>, &'static str> {
        let length = vector.len();

        if amount > length {
            return Err("can't get a sample bigger than the population");
        }

        let mut items: usize = 0;
        let mut positions: HashSet<usize> = HashSet::with_capacity(amount);
        let mut selected: Vec<&G> = Vec::with_capacity(amount);

        while items < amount {
            let pos = self.randrange(
                T::Number::zero()..T::Number::from_usize(length)
            ).to_usize();

            if !positions.contains(&pos) {
                positions.insert(pos);
                items += 1;
            }
        }

        for pos in &positions {
            selected.push(&vector[*pos]);
        }

        return Ok(selected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mersennetwister::MersenneTwister;

    #[test]
    fn randrange() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let value = rng.randrange(0..10);
        assert_eq!(value, 6);
    }

    #[test]
    fn random() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let value = rng.random();
        assert_eq!(value, 0.6555146273820462);
    }

    #[test]
    fn uniform() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let value = rng.uniform(1, 2);
        assert_eq!(value, 1.6555146273820462);
    }

    #[test]
    fn triangular() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let value = rng.triangular(1, 7, 4);
        assert_eq!(value, 4.5098721504462524);
    }

    #[test]
    fn randbytes() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let value = rng.randbytes(4);
        assert_eq!(value, vec![126, 210, 236, 124]);
    }

    #[test]
    fn choose() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let vector = vec![
            "This".to_string(), "is".to_string(), "a".to_string(), "test".to_string()
        ];
        let chosen = rng.choose(&vector);
        assert_eq!(chosen, "a");
    }

    #[test]
    fn shuffle() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let mut vector = vec![
            "This".to_string(), "is".to_string(), "a".to_string(), "test".to_string()
        ];
        rng.shuffle(&mut vector);
        assert_eq!(vector, vec![
            "is".to_string(), "This".to_string(), "test".to_string(), "a".to_string()
        ]);
    }

    #[test]
    fn sample() {
        let mut rng: Random<MersenneTwister> = Random::seed(10).unwrap();
        let vector = vec![
            "This".to_string(), "is".to_string(), "a".to_string(), "test".to_string()
        ];
        let sample = rng.sample(&vector, 2).unwrap();
        assert!(sample.len() == 2);
        assert_eq!(*sample[0], vector[0]);
        assert_eq!(*sample[1], vector[2]);
    }
}
