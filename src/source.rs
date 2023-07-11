use std::{
    iter::{Cloned, Cycle},
    slice::Iter,
};

pub fn integer_candidates() -> impl Iterator<Item = u64> {
    2..
}

pub fn odds_with_2() -> impl Iterator<Item = u64> {
    std::iter::once(2).chain((3..).step_by(2))
}

pub const WHEEL_2357_HOLES: [u64; 48] = [
    2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2, 4, 6,
    2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
];

pub type Wheel = Cycle<Cloned<Iter<'static, u64>>>;

pub fn wheel_2357() -> Wheel {
    WHEEL_2357_HOLES.iter().cloned().cycle()
}

pub struct SpinWheel<I> {
    wheel: I,
    n: u64,
}

impl<I> Iterator for SpinWheel<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.wheel.next()?;
        let n_res = self.n;
        self.n = n_res + x;
        return Some(n_res);
    }
}

impl Default for SpinWheel<Wheel> {
    fn default() -> Self {
        Self {
            wheel: wheel_2357(),
            n: 11,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::sieve::{test::PRIMES_100, TrialDivisionSieve},
        odds_with_2, SpinWheel,
    };

    #[test]
    fn test_odds_with_2() {
        assert!([2, 3, 5, 7, 9, 11, 13, 15, 17, 19]
            .iter()
            .cloned()
            .eq(odds_with_2().take(10)));
    }

    #[test]
    fn test_with_spin_wheel() {
        assert!([2, 3, 5, 7]
            .iter()
            .cloned()
            .chain(TrialDivisionSieve::with_source(SpinWheel::default()))
            .take(100)
            .eq(PRIMES_100.iter().cloned().map(|x| x as u64)));
    }
}
