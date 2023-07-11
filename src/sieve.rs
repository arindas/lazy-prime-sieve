//! Module providing sieves for generating primes.

use std::{collections::BinaryHeap, iter, mem};

/// Non-Recursive Rust implementation of the classic Haskell recursive method for generating primes.
///
/// This implementation attempts to reproduce the following Haskell code:
/// ```text
/// primes = sieve [2..]
/// sieve (p : xs) = p : sieve [x | x <− xs, x ‘mod‘ p > 0]
/// ````
pub struct UnfaithfulSieve {
    source: Box<dyn Iterator<Item = u64>>,
}

impl UnfaithfulSieve {
    pub fn with_source<I>(source: I) -> Self
    where
        I: Iterator<Item = u64> + 'static,
    {
        Self {
            source: Box::new(source),
        }
    }
}

impl Iterator for UnfaithfulSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut source = mem::replace(&mut self.source, Box::new(iter::empty()));
        let next_prime = source.next()?;
        self.source = Box::new(source.filter(move |x| x % next_prime > 0));
        Some(next_prime)
    }
}

/// The modulus based memoized approach of generating primes that we all know and love.
pub struct TrialDivisionSieve<I> {
    source: I,
    primes: Vec<u64>,
}

impl<I> TrialDivisionSieve<I> {
    pub fn with_source(source: I) -> Self {
        Self {
            source,
            primes: Vec::new(),
        }
    }
}

impl<I> Iterator for TrialDivisionSieve<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(candidate) = self.source.next() {
            if self
                .primes
                .iter()
                .take_while(|&x| x * x <= candidate)
                .all(|x| candidate % x > 0)
            {
                self.primes.push(candidate);
                return Some(candidate);
            }
        }

        None
    }
}

/// Creates an Iterator of integer multiples from the given iterator.
#[derive(Clone, Copy)]
pub enum IterMultiple<I> {
    Identity { source: I },
    Multiple { source: I, factor: u64 },
}

impl<I> IterMultiple<I> {
    pub fn multiply(self, factor: u64) -> Self {
        match self {
            IterMultiple::Identity { source } => IterMultiple::Multiple { source, factor },
            IterMultiple::Multiple { source, factor: f } => IterMultiple::Multiple {
                source,
                factor: f * factor,
            },
        }
    }
}

impl<I> Iterator for IterMultiple<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMultiple::Identity { source } => source.next(),
            IterMultiple::Multiple { source, factor: f } => source.next().map(|x| x * *f),
        }
    }
}

/// Table entry for genuine sieve of eratosthenes.
pub struct Entry<I> {
    pub key: u64,
    pub composites: IterMultiple<I>,
}

impl<I> PartialEq for Entry<I> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<I> PartialOrd for Entry<I> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.key.partial_cmp(&self.key)
    }
}

impl<I> Eq for Entry<I> {}

impl<I> Ord for Entry<I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.key.cmp(&self.key)
    }
}

/// Table for maintaining composites in the genuine prime sieve.
pub type Table<I> = BinaryHeap<Entry<I>>;

/// Genuine Sieve of eratosthenes implementation based on the paper:
/// [The Genuine Sieve of Eratosthenes](https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf)
pub struct GenuineSieve<I> {
    source: IterMultiple<I>,
    table: Table<I>,
}

impl<I> GenuineSieve<I> {
    pub fn with_source(source: I) -> Self {
        Self {
            source: IterMultiple::Identity { source },
            table: Table::new(),
        }
    }
}

impl<I> GenuineSieve<I>
where
    I: Iterator<Item = u64>,
{
    fn adjust_table(&mut self, candidate: u64) -> Option<()> {
        loop {
            match self.table.peek() {
                Some(Entry { key, composites: _ }) if key <= &candidate => {
                    let entry = self.table.pop()?;
                    let mut composites = entry.composites;
                    self.table.push(Entry {
                        key: composites.next()?,
                        composites,
                    });
                }
                _ => break,
            }
        }

        None
    }
}

impl<I> Iterator for GenuineSieve<I>
where
    I: Iterator<Item = u64> + Clone,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(candidate) = self.source.next() {
            match self.table.peek() {
                Some(Entry {
                    key: next_composite,
                    composites: _,
                }) if next_composite <= &candidate => self.adjust_table(candidate),
                _ => {
                    self.table.push(Entry {
                        key: candidate * candidate,
                        composites: self.source.clone().multiply(candidate),
                    });
                    return Some(candidate);
                }
            };
        }
        None
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{GenuineSieve, TrialDivisionSieve, UnfaithfulSieve};

    pub(crate) const PRIMES_100: [u16; 100] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541,
    ];

    #[test]
    fn test_unfaithful_sieve() {
        assert!(UnfaithfulSieve::with_source(2..)
            .take(100)
            .eq(PRIMES_100.iter().cloned().map(|x| x as u64)));
    }

    #[test]
    fn test_trial_division_sieve() {
        assert!(TrialDivisionSieve::with_source(2..)
            .take(100)
            .eq(PRIMES_100.iter().cloned().map(|x| x as u64)));
    }

    #[test]
    fn test_genuine_prime_sieve() {
        assert!(GenuineSieve::with_source(2..)
            .take(100)
            .eq(PRIMES_100.iter().cloned().map(|x| x as u64)));
    }
}
