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

# Run all tests
cargo test --all-features --workspace

# Run a specific test (exact name match)
cargo test test_vdcorput

# Run tests for a specific module
cargo test --lib lds

# Run tests with module prefix filter
cargo test ilds::tests::test_div_mod
cargo test sphere_n::tests::test_sphere3_basic

# Run tests with substring match
cargo test div_mod
cargo test sphere3
```

### Code Quality Checks
```bash
# Format code
cargo fmt --all

# Check formatting (CI uses this)
cargo fmt --all -- --check

# Run Clippy
cargo clippy --all-targets --all-features --workspace

# Generate and check documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

### Debugging Tips
```bash
# Show test output
cargo test -- --nocapture

# Run one test with output
cargo test test_vdcorput -- --nocapture --show-output

# Print ignored tests
cargo test -- --ignored

# Run tests with backtrace on failure
cargo test -- --show-output
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
use std::sync::atomic::{AtomicU32, Ordering};

// Then external crates (only in tests)
#[cfg(test)]
use approx::assert_relative_eq;
```

### Dependency Guidance
- `approx` crate is only used in tests - never in production code
- Use `#[cfg(test)]` for test-specific imports
- Avoid unnecessary dependencies - prefer std library

### Generator API Pattern

All sequence generators follow this consistent API pattern:

```rust
/// Brief description with context
///
/// More detailed explanation of purpose and usage.
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
    // Thread-safe counter
    count: AtomicU32,
    base: u32,
}

impl TypeName {
    /// Creates a new generator
    pub fn new(params) -> Self { /* ... */ }

    /// Returns the next value in the sequence
    pub fn pop(&mut self) -> OutputType {
        let idx = self.count.fetch_add(1, Ordering::Relaxed);
        // ... use idx to compute next value
    }

    /// Resets the sequence to a specific starting point
    pub fn reseed(&mut self, seed: u32) { /* ... */ }
}
```

### Thread-Safety Requirements
- All generators use `AtomicU32` for thread-safe state management
- Use `Ordering::Relaxed` for counters where cross-thread ordering isn't required
- Use `Ordering::SeqCst` only when memory ordering guarantees are needed
- Example pattern:
```rust
pub fn pop(&mut self) -> f64 {
    let idx = self.count.fetch_add(1, Ordering::Relaxed);
    // Compute value using idx
}
```

### Performance Considerations
- Atomic operations have overhead, but necessary for thread-safety
- Use `Ordering::Relaxed` for simple counters (most cases)
- Avoid `Ordering::SeqCst` unless absolutely needed (expensive)
- Cache precomputed values where beneficial (e.g., `rev_lst` in `VdCorput`)

### Documentation Standards
- Every public item must have `///` doc comments
- Include `# Examples` with runnable code
- Explain mathematical concepts clearly
- Reference related types where appropriate

## Testing Guidelines

### Test Structure
```rust
#[test]
fn test_type_name_feature() {
    // Setup
    let mut gen = TypeName::new(params);
    gen.reseed(0);

    // Execute
    let result = gen.pop();

    // Assert
    assert_relative_eq!(result, expected, epsilon = 1e-10);
}
```

### Floating-Point Comparisons
- Use `assert_relative_eq!` from the `approx` crate
- Always specify `epsilon` for tolerance (typically `1e-10`)
- Do not use `assert_eq!` for floating-point values
- Example:
```rust
assert_relative_eq!(result, expected, epsilon = 1e-10);
```

### Test Naming
- Format: `test_<type>_<feature>` or `test_<function>_behavior`
- Be descriptive: `test_sphere3_basic`, `test_vdcorput_first_values`
- Group related tests by prefix: `test_ilds_*`, `test_sphere_n_*`

## Module Organization

- **`lib.rs`**: Main library entry point, exports all public APIs
- **`lds.rs`**: Core floating-point sequence implementations
- **`ilds.rs`**: Integer sequence implementations
- **`sphere_n.rs`**: N-dimensional sphere sequences

## Common Patterns

### Constants
- Define math constants at module level using `pub const`
- Use `std::f64::consts` where possible (e.g., `PI`, `TAU`)
- Custom constants: `pub const TWO_PI: f64 = 2.0 * PI;`

### Dead Code Warnings
- Use `#[allow(dead_code)]` for fields kept for API consistency or documentation
- Example: `#[allow(dead_code)]` on `scale` field in integer sequences

### No Explicit Error Handling
- Generators don't return `Result` or `Option` (state-based generators always succeed)
- No panic paths in normal operation
- Input validation happens at construction time if needed

## Notes for Agents

- This is a Rust **2024 edition** project
- No custom `rustfmt.toml` or `clippy.toml` - use defaults
- The project emphasizes thread-safety and mathematical correctness
- Focus on maintaining the clean, documented API pattern across new implementations
- Refer to existing implementations (e.g., `VdCorput`, `Halton`) for patterns when adding new sequence types
- Always verify thread-safety when modifying generator state
