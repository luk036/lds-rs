# Gemini Code Understanding

## Project Overview

This project, `lds-rs`, is a Rust library for generating low-discrepancy sequences. These sequences are more evenly distributed than random numbers and are useful in applications like computer graphics, numerical integration, and Monte Carlo simulations.

The library provides several sequence generators:

*   Van der Corput
*   Halton
*   Circle
*   Disk
*   Sphere
*   3-Sphere Hopf
*   N-dimensional Halton

The core of the library is the Van der Corput sequence, which is extended to multiple dimensions and mapped to various geometric shapes.

## Building and Running

### Installation

To use `lds-rs` in your own Rust project, add it as a dependency in your `Cargo.toml` file.

You can also install it directly from crates.io:

```bash
cargo install lds-rs
```

### Running Tests

To run the tests for this project, use the following command:

```bash
cargo test
```

## Development Conventions

### Code Style

The code follows standard Rust conventions. It is well-documented with examples for each sequence generator.

### Testing

The project has a suite of tests that verify the correctness of the sequence generators. The tests use the `approx_eq` crate to compare floating-point numbers.

### Contributions

Contributions are welcome. See `CONTRIBUTING.md` for more information.
