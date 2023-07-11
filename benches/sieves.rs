use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lazy_prime_sieve::{
    sieve::{GenuineSieve, TrialDivisionSieve, UnfaithfulSieve},
    source::{integer_candidates, odds_with_2, SpinWheel},
};
use paste::paste;

macro_rules! sieve_functions {
    ($sieve_name:ident, $sieve_func_prefix:ident) => {
        paste! {
            fn [<$sieve_func_prefix _with_integer_candidates>](num_primes: usize) {
                $sieve_name::with_source(integer_candidates())
                    .take(num_primes)
                    .count();

            }

            fn [<$sieve_func_prefix _with_odds_with_2>](num_primes: usize) {
                $sieve_name::with_source(odds_with_2())
                    .take(num_primes)
                    .count();

            }

            fn [<$sieve_func_prefix _with_spin_wheel_2357>](num_primes: usize) {
                [2, 3, 5, 7]
                    .iter()
                    .cloned()
                    .chain($sieve_name::with_source(SpinWheel::default()))
                    .take(num_primes)
                    .count();

            }

        }
    };
}

sieve_functions!(UnfaithfulSieve, unfaithful_sieve);
sieve_functions!(TrialDivisionSieve, trial_division_sieve);
sieve_functions!(GenuineSieve, genuine_sieve);

macro_rules! bench_group {
    ($sieve_func_prefix:ident, $group:ident, $i:ident) => {
        paste! {
            $group.bench_with_input(BenchmarkId::new(stringify!([<$sieve_func_prefix _with_integer_candidates>]), $i), &$i,
                |b, i| b.iter(|| [<$sieve_func_prefix _with_integer_candidates>](*i)));
            $group.bench_with_input(BenchmarkId::new(stringify!([<$sieve_func_prefix _with_odds_with_2>]), $i), &$i,
                |b, i| b.iter(|| [<$sieve_func_prefix _with_odds_with_2>](*i)));
            $group.bench_with_input(BenchmarkId::new(stringify!([<$sieve_func_prefix _with_spin_wheel_2357>]), $i), &$i,
                |b, i| b.iter(|| [<$sieve_func_prefix _with_spin_wheel_2357>](*i)));
        }

    };
}

fn bench_sieves(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime-sieves");

    for i in (0..=10000_usize).step_by(1000) {
        bench_group!(unfaithful_sieve, group, i);
        bench_group!(trial_division_sieve, group, i);
        bench_group!(genuine_sieve, group, i);
    }
}

criterion_group!(benches, bench_sieves);
criterion_main!(benches);
