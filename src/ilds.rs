//! Integer Low-Discrepancy Sequence (ILDS) Generator
//!
//! This module implements integer versions of low-discrepancy sequence generators:
//! the van der Corput sequence and the Halton sequence for integer output.
//! These sequences are used to generate evenly distributed points in a space,
//! which can be useful for various applications like sampling, optimization,
//! or numerical integration.

use std::sync::atomic::{AtomicU64, Ordering};

/// Maximum number of digits for integer van der Corput sequence
const MAX_DIGITS: usize = 64;

/// Integer van der Corput sequence generator
///
/// Generates integer values of the van der Corput sequence with a specified scale.
/// Unlike floating-point VdCorput, this takes a `scale` parameter because
/// integer output requires knowing the maximum value range.
///
/// # Examples
///
/// ```
/// use lds_gen::ilds::VdCorput;
/// let mut vdc = VdCorput::new(2, 10);
/// vdc.reseed(0);
/// assert_eq!(vdc.pop(), 512); // 0.5 * 2^10 = 512
/// ```
#[derive(Debug)]
pub struct VdCorput {
    base: u64,
    count: AtomicU64,
    factor_lst: Vec<u64>,
}

impl VdCorput {
    /// Creates a new integer van der Corput sequence generator
    ///
    /// # Arguments
    ///
    /// * `base` - The base of the number system (defaults to 2 if not specified)
    /// * `scale` - The scale factor determining the number of digits that can be represented
    pub fn new(base: u64, scale: u32) -> Self {
        let mut factor = 1u64;
        let n = (scale as usize).min(MAX_DIGITS);
        let mut factor_lst = vec![0u64; MAX_DIGITS];
        for i in 0..n {
            factor_lst[n - 1 - i] = factor;
            factor = factor.checked_mul(base).expect("scale too large");
        }
        Self {
            base,
            count: AtomicU64::new(0),
            factor_lst,
        }
    }

    /// Generates the next integer value in the sequence
    ///
    /// $$ \phi_b(n) = \sum_{k=0}^{m} d_k \cdot \frac{b^{\text{scale}}}{b^{k+1}} $$
    ///
    /// Increments the count and calculates the next integer value
    /// in the van der Corput sequence.
    pub fn pop(&mut self) -> u64 {
        let count = self.count.fetch_add(1, Ordering::Relaxed) + 1;
        let mut count = count;
        let mut reslt = 0;
        let mut idx = 0;

        while count != 0 {
            let remainder = count % self.base;
            count /= self.base;
            reslt += remainder * self.factor_lst[idx];
            idx += 1;
        }
        reslt
    }

    /// Returns the next value without advancing the state (peek)
    ///
    /// $$ \phi_b(n) = \sum_{k=0}^{m} d_k \cdot \frac{b^{\text{scale}}}{b^{k+1}} $$
    pub fn peek(&self) -> u64 {
        let mut count = self.count.load(Ordering::Relaxed) + 1;
        let mut reslt = 0;
        let mut idx = 0;

        while count != 0 {
            let remainder = count % self.base;
            count /= self.base;
            reslt += remainder * self.factor_lst[idx];
            idx += 1;
        }
        reslt
    }

    /// Advances the sequence by `n` values without computing them
    ///
    /// # Arguments
    ///
    /// * `n` - The number of values to advance
    pub fn advance(&self, n: u64) {
        self.count.fetch_add(n, Ordering::Relaxed);
    }

    /// Returns the current index (number of values generated so far)
    pub fn get_index(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Resets the state of the sequence generator to a specific seed value
    ///
    /// # Arguments
    ///
    /// * `seed` - The seed value that determines the starting point of the sequence generation
    pub fn reseed(&mut self, seed: u64) {
        self.count.store(seed, Ordering::Relaxed);
    }
}

impl Default for VdCorput {
    /// Creates a default integer van der Corput generator
    ///
    /// Defaults to base 2 with scale 10 (produces values in range [0, 1024))
    fn default() -> Self {
        Self::new(2, 10)
    }
}

impl Iterator for VdCorput {
    type Item = u64;

    /// Returns the next value in the sequence
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pop())
    }
}

/// Integer Halton sequence generator
///
/// Generates points in a 2-dimensional space using integer Halton sequences.
///
/// # Examples
///
/// ```
/// use lds_gen::ilds::Halton;
/// let mut hgen = Halton::new([2, 3], [11, 7]);
/// hgen.reseed(0);
/// let res = hgen.pop();
/// assert_eq!(res[0], 1024); // 0.5 * 2^11 = 1024
/// assert_eq!(res[1], 729);  // 1/3 * 3^7 = 729
/// ```
pub struct Halton {
    vdc0: VdCorput,
    vdc1: VdCorput,
}

impl Halton {
    /// Creates a new integer Halton sequence generator with the given bases and scales
    ///
    /// # Arguments
    ///
    /// * `base` - An array of two integers used as bases for generating the sequence
    /// * `scale` - An array of two integers used as scales for each dimension
    pub fn new(base: [u64; 2], scale: [u32; 2]) -> Self {
        Self {
            vdc0: VdCorput::new(base[0], scale[0]),
            vdc1: VdCorput::new(base[1], scale[1]),
        }
    }

    /// Generates the next point in the integer Halton sequence
    ///
    /// Returns the next point as a `[u64; 2]`.
    pub fn pop(&mut self) -> [u64; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /// Resets the state of the sequence generator to a specific seed value
    ///
    /// # Arguments
    ///
    /// * `seed` - The seed value that determines the starting point of the sequence generation
    pub fn reseed(&mut self, seed: u64) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}

impl Iterator for Halton {
    type Item = [u64; 2];

    /// Returns the next point in the integer Halton sequence
    ///
    /// This allows Halton to be used with iterator methods like `.take()`, `.collect()`, etc.
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ilds_vdcorput_pop() {
        let mut vdc = VdCorput::new(2, 10);
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 512); // 0.5 * 1024
        assert_eq!(vdc.pop(), 256); // 0.25 * 1024
        assert_eq!(vdc.pop(), 768); // 0.75 * 1024
        assert_eq!(vdc.pop(), 128); // 0.125 * 1024
    }

    #[test]
    fn test_ilds_vdcorput_reseed() {
        let mut vdc = VdCorput::new(2, 10);
        vdc.reseed(5);
        assert_eq!(vdc.pop(), 384); // 0.375 * 1024
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 512); // 0.5 * 1024
    }

    #[test]
    fn test_ilds_vdcorput_default() {
        let mut vdc = VdCorput::default();
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 512);
        assert_eq!(vdc.pop(), 256);
    }

    #[test]
    fn test_ilds_halton_pop() {
        let mut hgen = Halton::new([2, 3], [11, 7]);
        hgen.reseed(0);
        let res = hgen.pop();
        assert_eq!(res[0], 1024); // 0.5 * 2048
        assert_eq!(res[1], 729); // 1/3 * 2187

        let res = hgen.pop();
        assert_eq!(res[0], 512); // 0.25 * 2048
        assert_eq!(res[1], 1458); // 2/3 * 2187
    }

    // Additional comprehensive tests for edge cases and different scales

    #[test]
    fn test_ilds_vdcorput_different_bases() {
        // Test with base 3
        let mut vdc = VdCorput::new(3, 5);
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 81); // Actual value from implementation
        assert_eq!(vdc.pop(), 162); // Next value

        // Test with base 5
        let mut vdc = VdCorput::new(5, 3);
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 25); // 0.2 * 5^3 = 62.5 -> 62 (integer)
        assert_eq!(vdc.pop(), 50); // Next value
    }

    #[test]
    fn test_ilds_vdcorput_different_scales() {
        // Test with different scales
        let mut vdc1 = VdCorput::new(2, 5);
        vdc1.reseed(0);
        assert_eq!(vdc1.pop(), 16); // 0.5 * 2^5 = 16

        let mut vdc2 = VdCorput::new(2, 10);
        vdc2.reseed(0);
        assert_eq!(vdc2.pop(), 512); // 0.5 * 2^10 = 512

        let mut vdc3 = VdCorput::new(2, 15);
        vdc3.reseed(0);
        assert_eq!(vdc3.pop(), 16384); // 0.5 * 2^15 = 16384
    }

    #[test]
    fn test_ilds_vdcorput_large_values() {
        let mut vdc = VdCorput::new(2, 20);
        vdc.reseed(1000);

        // Generate several values and ensure they're within valid range
        for _ in 0..10 {
            let value = vdc.pop();
            assert!(value < vdc.factor_lst[0] * vdc.base);
        }
    }

    #[test]
    fn test_ilds_halton_different_bases_and_scales() {
        // Test with bases 3 and 5, scales 5 and 7
        let mut hgen = Halton::new([3, 5], [5, 7]);
        hgen.reseed(0);
        let res = hgen.pop();
        assert_eq!(res[0], 81); // Actual value from VdCorput with base 3, scale 5
        assert_eq!(res[1], 15625); // Actual value from VdCorput with base 5, scale 7

        // Test with bases 5 and 7, scales 3 and 4
        let mut hgen = Halton::new([5, 7], [3, 4]);
        hgen.reseed(0);
        let res = hgen.pop();
        assert_eq!(res[0], 25); // Actual value from VdCorput with base 5, scale 3
        assert_eq!(res[1], 343); // Actual value from VdCorput with base 7, scale 4
    }

    #[test]
    fn test_ilds_halton_large_values() {
        let mut hgen = Halton::new([2, 3], [15, 10]);
        hgen.reseed(0);

        // Generate several values and ensure they're within valid range
        for _ in 0..10 {
            let res = hgen.pop();
            assert!(res[0] < hgen.vdc0.factor_lst[0] * hgen.vdc0.base);
            assert!(res[1] < hgen.vdc1.factor_lst[0] * hgen.vdc1.base);
        }
    }

    #[test]
    fn test_ilds_sequence_properties() {
        // Test that ILDS VdCorput sequence values are always within valid range
        let mut vdc = VdCorput::new(2, 10);
        for _ in 0..100 {
            let value = vdc.pop();
            assert!(value < vdc.factor_lst[0] * vdc.base);
        }

        // Test that ILDS Halton sequence values are always within valid range
        let mut hgen = Halton::new([2, 3], [10, 8]);
        for _ in 0..100 {
            let res = hgen.pop();
            assert!(res[0] < hgen.vdc0.factor_lst[0] * hgen.vdc0.base);
            assert!(res[1] < hgen.vdc1.factor_lst[0] * hgen.vdc1.base);
        }
    }

    #[test]
    fn test_ilds_reseed_consistency() {
        // Test that reseed with the same value produces the same sequence
        let mut vdc = VdCorput::new(2, 10);

        vdc.reseed(10);
        let seq1: Vec<_> = (0..5).map(|_| vdc.pop()).collect();

        vdc.reseed(10);
        let seq2: Vec<_> = (0..5).map(|_| vdc.pop()).collect();

        assert_eq!(seq1, seq2);

        // Test that reseed with different values produces different sequences
        vdc.reseed(10);
        let seq3: Vec<_> = (0..5).map(|_| vdc.pop()).collect();

        vdc.reseed(20);
        let seq4: Vec<_> = (0..5).map(|_| vdc.pop()).collect();

        assert_ne!(seq3, seq4);
    }

    #[test]
    fn test_ilds_default_implementation() {
        // Test Default for VdCorput
        let mut vdc = VdCorput::default();
        vdc.reseed(0);
        assert_eq!(vdc.pop(), 512); // Default is base=2, scale=10

        // Verify default parameters
        let vdc_default = VdCorput::default();
        assert_eq!(vdc_default.base, 2);
        assert_eq!(vdc_default.factor_lst[0] * vdc_default.base, 1024);
    }

    #[test]
    fn test_ilds_vdcorput_peek() {
        let mut vdc = VdCorput::new(2, 10);
        vdc.reseed(0);
        let peeked = vdc.peek();
        assert_eq!(peeked, 512);
        let popped = vdc.pop();
        assert_eq!(popped, 512); // peek doesn't advance
        assert_eq!(vdc.peek(), 256);
    }

    #[test]
    fn test_ilds_vdcorput_advance() {
        let mut vdc = VdCorput::new(2, 10);
        vdc.reseed(0);
        vdc.advance(3);
        assert_eq!(vdc.pop(), 128); // 0.125 * 1024
        vdc.reseed(0);
        vdc.advance(4);
        // vdc_i(5, 2) = binary 101 → 1*512 + 0*256 + 1*128 = 640 (0.625 * 1024)
        assert_eq!(vdc.pop(), 640);
    }

    #[test]
    fn test_ilds_vdcorput_get_index() {
        let mut vdc = VdCorput::new(2, 10);
        assert_eq!(vdc.get_index(), 0);
        vdc.pop();
        assert_eq!(vdc.get_index(), 1);
        vdc.pop();
        assert_eq!(vdc.get_index(), 2);
        vdc.reseed(5);
        assert_eq!(vdc.get_index(), 5);
    }

    #[test]
    fn test_ilds_halton_iterator() {
        let mut hgen = Halton::new([2, 3], [11, 7]);
        hgen.reseed(0);
        let values: Vec<[u64; 2]> = hgen.take(3).collect();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0][0], 1024);
        assert_eq!(values[0][1], 729);
        assert_eq!(values[1][0], 512);
        assert_eq!(values[1][1], 1458);
    }
}
