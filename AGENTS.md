# AGENTS.md

This file provides guidelines for agentic coding assistants working on the `lds-rs` project.

## Project Overview

`lds-rs` is a Rust library for generating Low-Discrepancy Sequences (LDS), used in quasi-Monte Carlo methods, numerical integration, and Monte Carlo simulations. All generators are thread-safe using atomic operations.

## Build, Lint, and Test Commands

### Basic Operations
```bash
# Build release version
cargo build --release

# Run release binary
cargo run --release

# Run all tests (including doc tests)
cargo test --all-features --workspace

# Run a specific test (exact name match)
cargo test test_vdcorput

# Run tests for a specific module
cargo test --lib lds

# Run tests with substring match
cargo test div_mod
cargo test sphere3
```

### Feature Flags
```bash
# Run tests with specific features
cargo test --features tracing
cargo test --features env_logger

# Run all tests with all features
cargo test --all-features --workspace --lib --examples --doc
```

### Code Quality Checks
```bash
# Format code
cargo fmt --all

# Check formatting (CI uses this)
cargo fmt --all -- --check

# Run Clippy
cargo clippy --all-targets --all-features --workspace

# Generate documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

### Debugging Tips
```bash
# Show test output
cargo test -- --nocapture

# Run one test with output
cargo test test_vdcorput -- --nocapture --show-output

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test
```

### Pre-commit Checklist
Before submitting changes:
1. Run full test suite: `cargo test --all-features --workspace`
2. Check formatting: `cargo fmt --all -- --check`
3. Run Clippy: `cargo clippy --all-targets --all-features --workspace`
4. Check docs: `cargo doc --no-deps --document-private-items --all-features --workspace --examples`

## Code Style Guidelines

### Naming Conventions
- **Types/Structs**: `PascalCase` (e.g., `VdCorput`, `Halton`, `Sphere`)
- **Functions/Methods**: `snake_case` (e.g., `pop()`, `reseed()`, `vdc()`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `TWO_PI`, `PRIME_TABLE`)
- **Private fields**: `snake_case` (e.g., `count`, `base`, `rev_lst`)
- **Modules**: `snake_case` (e.g., `ilds`, `sphere_n`)

### Import Organization
```rust
// Standard library imports first
use std::f64::consts::PI;
use std::sync::atomic::{AtomicU64, Ordering};

// External crates (only in tests)
#[cfg(test)]
use approx::assert_relative_eq;
```

### Dependency Guidance
- `approx` crate: only in `[dev-dependencies]` - never in production code
- Use `#[cfg(test)]` for test-specific imports
- Avoid unnecessary dependencies - prefer std library

### Generator API Pattern

All sequence generators follow this consistent API:

```rust
/// Brief description with context
///
/// # Examples
///
/// ```
/// use lds_gen::TypeName;
/// let mut gen = TypeName::new(params);
/// gen.reseed(0);
/// let result = gen.pop();
/// ```
#[derive(Debug)]
pub struct TypeName {
    count: AtomicU64,  // Thread-safe counter
    base: u64,
}

impl TypeName {
    pub fn new(params) -> Self { /* ... */ }
    pub fn pop(&mut self) -> f64 {
        let idx = self.count.fetch_add(1, Ordering::Relaxed);
        // Compute value using idx
    }
    pub fn reseed(&mut self, seed: u64) { /* ... */ }
}
```

### Thread-Safety Requirements
- Use `AtomicU64` for thread-safe state management
- Use `Ordering::Relaxed` for counters (most cases)
- Avoid `Ordering::SeqCst` unless absolutely needed
- Cache precomputed values (e.g., `rev_lst` in `VdCorput`)

### Documentation Standards
- Every public item needs `///` doc comments
- Include `# Examples` with runnable code
- Reference related types where appropriate

### Dead Code Warnings
- Use `#[allow(dead_code)]` for fields kept for API consistency
- Example: `#[allow(dead_code)]` on `scale` field in integer sequences

## Testing Guidelines

### Test Structure
```rust
#[test]
fn test_type_name_feature() {
    let mut gen = TypeName::new(params);
    gen.reseed(0);
    let result = gen.pop();
    assert_relative_eq!(result, expected, epsilon = 1e-10);
}
```

### Floating-Point Comparisons
- Use `assert_relative_eq!` from `approx` crate
- Always specify `epsilon` (typically `1e-10`)
- Never use `assert_eq!` for floating-point values

### Test Naming
- Format: `test_<type>_<feature>` or `test_<function>_behavior`
- Group related tests by prefix: `test_ilds_*`, `test_sphere_n_*`

## Module Organization

- **`lib.rs`**: Main library entry point, exports all public APIs
- **`ilds.rs`**: Integer sequence implementations (VdCorput, Halton)
- **`sphere_n.rs`**: N-dimensional sphere sequences

## Error Handling Philosophy

- Generators don't return `Result` or `Option` - state-based generators always succeed
- No panic paths in normal operation
- Input validation happens at construction time (`assert!` in `new()`)

## Notes for Agents

- This is a Rust **2024 edition** project
- No custom `rustfmt.toml` or `clippy.toml` - use defaults
- Project emphasizes thread-safety and mathematical correctness
- Refer to existing implementations (`VdCorput`, `Halton`, `Sphere`) for patterns
- Always verify thread-safety when modifying generator state