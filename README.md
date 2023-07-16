<p align="center">
<h1 align="center"><code>lazy-prime-sieve</code></h1>
</p>

<p align="center">
  <a href="https://github.com/arindas/lazy-prime-sieve/actions/workflows/rust-ci.yml">
    <img src="https://github.com/arindas/lazy-prime-sieve/actions/workflows/rust-ci.yml/badge.svg">
  </a>
  <a href="https://codecov.io/gh/arindas/lazy-prime-sieve">
    <img src="https://codecov.io/gh/arindas/lazy-prime-sieve/branch/main/graph/badge.svg?token=MZfko4wvwc"/>
  </a>
  <a href="https://crates.io/crates/lazy-prime-sieve">
  <img src="https://img.shields.io/crates/v/lazy-prime-sieve" />
  </a>
  <a href="https://github.com/arindas/lazy-prime-sieve/actions/workflows/rustdoc.yml">
  <img src="https://github.com/arindas/lazy-prime-sieve/actions/workflows/rustdoc.yml/badge.svg" />
  </a>
</p>

<p align="center">
Lazy Sieve of Eratosthenes for infinitely generating primes lazily in Rust.
</p>

## Usage

`lazy-prime-sieve` is a library crate. You may add it to your `Cargo.toml` as
follows:

```toml
[dependencies]
lazy-prime-sieve = "0.1.3"
```

`lazy-prime-sieve` provides iterators for infinitely generating primes. This
crate provides a convenience method `::primes()` which returns the most
efficient iterator (in this crate) for generating primes.


```rust
use lazy_prime_sieve::primes;

for i in primes().take(10) {
    println!("{i}");
}
```

## Design

This crate provides two types of abstractions: `sieve`(s) and `source`(s).
- `source`(s) represent infinite sources of integers from which we sample primes.
- `sieve`(s) sample primes from `source`(s).

Both `source`(s) and `sieve`(s) implement `Iterator<Item = u64>`.

This crate provides the following sieves:
- `UnfaithfulSieve`: Non-recursive `Iterator` based alternative to classic Haskell
lazy recursive prime sieve:
  ```haskell
  primes = sieve [2..]
  sieve (p : xs) = p : sieve [x | x <− xs, x ‘mod‘ p > 0]
  ```
- `TrialDivsionSieve`: The modulus-based memoized approach of generating primes
that we all know and love.
- `GenuineSieve`: Priority Queue based solution, true to the original Sieve of
Eratosthenes algorithm.

This crate provides the following sources:
- `integer_candidates()`: Returns an iterator which yields the sequence 2, 3, 4, 5, 6, 7, …
- `odds_with_2()`: Returns an iterator which yields the sequence 2, 3, 5, 7, 9, …
- `SpinWheel::default()`: Iterator of monotonically increasing integers which are not
multiples of 2, 3, 5 and 7.

Mostly, we initialize a `sieve` with a `source` and start generating primes:

```rust
use lazy_prime_sieve::{sieve::TrialDivisionSieve, source::odds_with_2};

// print the first 10 primes
TrialDivisionSieve::with_source(odds_with_2())
  .take(10)
  .for_each(|x| println!("{x}"));
```

However, some sources start from a high number. So we need to chain the initial
primes:

```rust
use lazy_prime_sieve::{source::SpinWheel, sieve::GenuineSieve};

// starts from 11
let source = SpinWheel::default();

// print the first 10 primes
[2, 3, 5, 7]
    .iter()
    .cloned()
    .chain(GenuineSieve::with_source(source))
    .take(10)
    .for_each(|x| println!("{x}"));
```

Refer to the [documentation](https://docs.rs/lazy-prime-sieve/) for further
details.

## Benchmarks

![prime-sieves-bench](https://raw.githubusercontent.com/arindas/lazy-prime-sieve/main/assets/lines.svg)

This benchmark shows the time taken by the different `(source, sieve)`
combinations (fmt: `"{sieve}_with_{source}"`) in this crate to generate a
certain number of primes. The `x-axis` shows the number of primes generated,
while the `y-axis` shows the time taken.

The fastest combination is `GenuineSieve` with `SpinWheel::default()`. This is
the combination used by `lazy_prime_sieve::primes()`.

See the generated benchmark report [here](https://arindas.github.io/lazy-prime-sieve/criterion/report/index.html).

These benchmarks were run on an AMD Ryzen 7 x86_64 machine in WSL with 8 GB RAM
allocated to WSL.

## References

This crate heavily draws from the paper [The Genuine Sieve of
Eratosthenes](https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf). This
repository attempts to provide non-recursive lazy Rust iterator based
alternatives to the Haskell lazy list + recursion based methods proposed in the
paper.

## License

`lazy-prime-sieve` is licensed under the MIT License. See
[License](https://raw.githubusercontent.com/arindas/lazy-prime-sieve/main/LICENSE)
for more details.
