use std::{iter, mem};

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

#[cfg(test)]
pub(crate) mod test {
    use super::{TrialDivisionSieve, UnfaithfulSieve};

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
}
