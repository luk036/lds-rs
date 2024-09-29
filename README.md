# lds-rs

[![Crates.io](https://img.shields.io/crates/v/lds-rs.svg)](https://crates.io/crates/lds-rs)
[![Docs.rs](https://docs.rs/lds-rs/badge.svg)](https://docs.rs/lds-rs)
[![CI](https://github.com/luk036/lds-rs/workflows/CI/badge.svg)](https://github.com/luk036/lds-rs/actions)
[![codecov](https://codecov.io/gh/luk036/lds-rs/branch/master/graph/badge.svg?token=wu6Alzj2TF)](https://codecov.io/gh/luk036/lds-rs)

This code implements a set of low-discrepancy sequence generators, which are used to create sequences of numbers that are more evenly distributed than random numbers. These sequences are particularly useful in various fields such as computer graphics, numerical integration, and Monte Carlo simulations.

The code defines several classes, each representing a different type of low-discrepancy sequence generator. The main types of sequences implemented are:

1. Van der Corput sequence
2. Halton sequence
3. Circle sequence
4. Sphere sequence
5. 3-Sphere Hopf sequence
6. N-dimensional Halton sequence

Each generator takes specific inputs, usually in the form of base numbers or sequences of base numbers. These bases determine how the sequences are generated. The generators produce outputs in the form of floating-point numbers or lists of floating-point numbers, depending on the dimensionality of the sequence.

The core algorithm used in most of these generators is the Van der Corput sequence. This sequence is created by expressing integers in a given base, reversing the digits, and placing them after a decimal point. For example, in base 2, the sequence would start: 1/2, 1/4, 3/4, 1/8, 5/8, and so on.

The Halton sequence extends this concept to multiple dimensions by using a different base for each dimension. The Circle and Sphere sequences use trigonometric functions to map these low-discrepancy sequences onto circular or spherical surfaces.

The code also includes utility functions and classes to support these generators. For instance, there's a list of prime numbers that can be used as bases for the sequences.

Each generator class has methods to produce the next value in the sequence (pop()) and to reset the sequence to a specific starting point (reseed()). This allows for flexible use of the generators in various applications.

The purpose of this code is to provide a toolkit for generating well-distributed sequences of numbers, which can be used in place of random numbers in many applications to achieve more uniform coverage of a given space or surface. This can lead to more efficient and accurate results in tasks like sampling, integration, and optimization.

## 🛠️ Installation

### 📦 Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install lds-rs`

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
