# iFlow Context File for lds-rs

## Project Overview

lds-rs is a Rust library for generating Low Discrepancy Sequences (LDS). This library provides various types of low-discrepancy sequence generators that create sequences of numbers with better uniformity than random numbers. These sequences are useful in fields like computer graphics, numerical integration, and Monte Carlo simulations.

The main sequence types implemented in this library are:
1. Van der Corput sequence
2. Halton sequence
3. Circle sequence
4. Sphere sequence
5. 3-Sphere Hopf sequence
6. N-dimensional Halton sequence

The library also includes integer-specific versions of these sequences in the `ilds` module.

## Key Components

- **src/lib.rs**: Main library module that exports all public types and functions
- **src/lds.rs**: Implementation of the main low-discrepancy sequence generators with floating-point output
- **src/ilds.rs**: Implementation of low-discrepancy sequence generators with integer output
- **Cargo.toml**: Project manifest containing dependencies and metadata

## Building and Running

### Prerequisites
- Rust toolchain (edition 2021)

### Build Commands
- `cargo build` - Build the library in debug mode
- `cargo build --release` - Build the library in release mode

### Test Commands
- `cargo test` - Run all tests
- `cargo test --release` - Run tests in release mode

### Installation
- `cargo install lds-rs` - Install the crate from crates.io

## Development Conventions

- The library uses the `tracing` crate for logging
- Tests are written using Rust's built-in test framework
- The `approx_eq` crate is used in tests for floating-point comparisons
- Prime numbers are pre-computed in a constant array for use as bases
- Each generator has `pop()` method to generate the next value and `reseed()` method to reset the sequence

## Public API

The library exports the following main types:
- `VdCorput` - Van der Corput sequence generator
- `Halton` - 2D Halton sequence generator
- `Circle` - Circle sequence generator
- `Disk` - Disk sequence generator
- `Sphere` - Sphere sequence generator
- `Sphere3Hopf` - 3-Sphere Hopf sequence generator
- `HaltonN` - N-dimensional Halton sequence generator
- `PRIME_TABLE` - Constant array of the first 1000 prime numbers