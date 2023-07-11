pub mod sieve;
pub mod source;

/// Returns an Iterator of prime numbers.
///
/// This function internally invokes a [sieve::GenuineSieve]
/// with a [source::SpinWheel] source.
pub fn primes() -> impl Iterator<Item = u64> {
    [2, 3, 5, 7]
        .iter()
        .cloned()
        .chain(sieve::GenuineSieve::with_source(
            source::SpinWheel::default(),
        ))
}

#[cfg(test)]
mod tests {
    use crate::{primes, sieve::test::PRIMES_100};

    #[test]
    fn primes_100() {
        assert!(primes()
            .take(100)
            .eq(PRIMES_100.iter().cloned().map(|x| x as u64)));
    }
}
