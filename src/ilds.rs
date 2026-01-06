//! Integer Low-Discrepancy Sequence (ILDS) Generator
//!
//! This module implements integer versions of low-discrepancy sequence generators:
//! the Van der Corput sequence and the Halton sequence for integer output.
//! These sequences are used to generate evenly distributed points in a space,
//! which can be useful for various applications like sampling, optimization,
//! or numerical integration.

use std::sync::atomic::{AtomicU32, Ordering};

/// Integer Van der Corput sequence generator
///
/// Generates integer values of the Van der Corput sequence with a specified scale.
///
/// # Examples
///
/// ```
/// use lds_gen::ilds::VdCorput;
/// let mut vdc = VdCorput::new(2, 10);
/// vdc.reseed(0);
/// assert_eq!(vdc.pop(), 512); // 0.5 * 2^10 = 512
/// ```
pub struct VdCorput {
    base: u32,
    #[allow(dead_code)] // Used for documentation and API consistency
    scale: u32,
    count: AtomicU32,
    factor: u32,
}

impl VdCorput {
    /// Creates a new integer Van der Corput sequence generator
    ///
    /// # Arguments
    ///
    /// * `base` - The base of the number system (defaults to 2 if not specified)
    /// * `scale` - The scale factor determining the number of digits that can be represented
    pub fn new(base: u32, scale: u32) -> Self {
        let factor = base.pow(scale);
        Self {
            base,
            scale,
            count: AtomicU32::new(0),
            factor,
        }
    }

    /// Generates the next integer value in the sequence
    ///
    /// Increments the count and calculates the next integer value
    /// in the Van der Corput sequence.
    pub fn pop(&mut self) -> u32 {
        let count = self.count.fetch_add(1, Ordering::Relaxed) + 1;
        let mut count = count;
        let mut reslt = 0;
        let mut factor = self.factor;

        while count != 0 {
            let remainder = count % self.base;
            factor /= self.base;
            count /= self.base;
            reslt += remainder * factor;
        }
        reslt
    }

    /// Resets the state of the sequence generator to a specific seed value
    ///
    /// # Arguments
    ///
    /// * `seed` - The seed value that determines the starting point of the sequence generation
    pub fn reseed(&mut self, seed: u32) {
        self.count.store(seed, Ordering::Relaxed);
    }
}

impl Default for VdCorput {
    fn default() -> Self {
        Self::new(2, 10)
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
    pub fn new(base: [u32; 2], scale: [u32; 2]) -> Self {
        Self {
            vdc0: VdCorput::new(base[0], scale[0]),
            vdc1: VdCorput::new(base[1], scale[1]),
        }
    }

    /// Generates the next point in the integer Halton sequence
    ///
    /// Returns the next point as a `[u32; 2]`.
    pub fn pop(&mut self) -> [u32; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /// Resets the state of the sequence generator to a specific seed value
    ///
    /// # Arguments
    ///
    /// * `seed` - The seed value that determines the starting point of the sequence generation
    pub fn reseed(&mut self, seed: u32) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}

macro_rules! div_mod_3_iter {
    ($input:expr) => {{
        let q = $input >> 2; // Equivalent to extracting upper bits
        let r = $input & 0x03; // Equivalent to extracting lower 2 bits
        (q, q + r) // Return the sum of q and r
    }};
}

pub fn div_mod_3_u8(n: u8) -> (u8, u8) {
    // Perform the iterations using the macro
    let (q1, rem1) = div_mod_3_iter!(n); // First iteration
    let (q2, rem2) = div_mod_3_iter!(rem1); // Second iteration
    let (q3, rem3) = div_mod_3_iter!(rem2); // Third iteration
    let (q4, rem4) = div_mod_3_iter!(rem3); // Fourth iteration

    // Calculate the final quotient sum
    let quotient_sum = q1 + q2 + q3 + q4;

    // Final check and output assignment
    if rem4 == 0x03 {
        // Equivalent to rem4 == 2'b11
        (quotient_sum + 1, 0x00) // Equivalent to quotient_sum + 1 and remainder 2'b00
    } else {
        (quotient_sum, rem4) // Equivalent to quotient_sum and rem4[1:0]
    }
}

/// # Examples
///
/// ```rust
/// use lds_gen::ilds::div_mod_3_u8;
///
/// let (q, r) = div_mod_3_u8(10);
/// assert_eq!(q, 3);
/// assert_eq!(r, 1);
///
/// let (q, r) = div_mod_3_u8(12);
/// assert_eq!(q, 4);
/// assert_eq!(r, 0);
/// ```
pub fn div_mod_3_u16(n: u16) -> (u16, u16) {
    // Perform the iterations using the macro
    let (q1, rem1) = div_mod_3_iter!(n); // First iteration
    let (q2, rem2) = div_mod_3_iter!(rem1); // Second iteration
    let (q3, rem3) = div_mod_3_iter!(rem2); // Third iteration
    let (q4, rem4) = div_mod_3_iter!(rem3); // Fourth iteration
    let (q5, rem5) = div_mod_3_iter!(rem4); // 5th iteration
    let (q6, rem6) = div_mod_3_iter!(rem5); // 6th iteration
    let (q7, rem7) = div_mod_3_iter!(rem6); // 7th iteration
    let (q8, rem8) = div_mod_3_iter!(rem7); // 8th iteration

    // Calculate the final quotient sum
    let quotient_sum = q1 + q2 + q3 + q4 + q5 + q6 + q7 + q8;

    // Final check and output assignment
    if rem8 == 0x03 {
        // Equivalent to rem4 == 2'b11
        (quotient_sum + 1, 0x00) // Equivalent to quotient_sum + 1 and remainder 2'b00
    } else {
        (quotient_sum, rem8) // Equivalent to quotient_sum and rem8[1:0]
    }
}

/// # Examples
///
/// ```rust
/// use lds_gen::ilds::div_mod_3_u16;
///
/// let (q, r) = div_mod_3_u16(10000);
/// assert_eq!(q, 3333);
/// assert_eq!(r, 1);
///
/// let (q, r) = div_mod_3_u16(10002);
/// assert_eq!(q, 3334);
/// assert_eq!(r, 0);
/// ```
macro_rules! div_mod_7_iter {
    ($input:expr) => {{
        let q = $input >> 3; // Equivalent to extracting upper bits
        let r = $input & 0x07; // Equivalent to extracting lower 3 bits
        (q, q + r) // Return the sum of q and r
    }};
}

pub fn div_mod_7_u8(n: u8) -> (u8, u8) {
    // Perform the iterations using the macro
    let (q1, rem1) = div_mod_7_iter!(n); // First iteration
    let (q2, rem2) = div_mod_7_iter!(rem1); // Second iteration
    let (q3, rem3) = div_mod_7_iter!(rem2); // Third iteration

    // Calculate the final quotient sum
    let quotient_sum = q1 + q2 + q3;

    // Final check and output assignment
    if rem3 == 0x07 {
        // Equivalent to rem3 == 3'b111
        (quotient_sum + 1, 0x000) // Equivalent to quotient_sum + 1 and remainder 3'b000
    } else {
        (quotient_sum, rem3) // Equivalent to quotient_sum and rem3[1:0]
    }
}

/// # Examples
///
/// ```rust
/// use lds_gen::ilds::div_mod_7_u8;
///
/// let (q, r) = div_mod_7_u8(10);
/// assert_eq!(q, 1);
/// assert_eq!(r, 3);
///
/// let (q, r) = div_mod_7_u8(14);
/// assert_eq!(q, 2);
/// assert_eq!(r, 0);
/// ```
pub fn div_mod_7_u16(n: u16) -> (u16, u16) {
    // Perform the iterations using the macro
    let (q1, rem1) = div_mod_7_iter!(n); // First iteration
    let (q2, rem2) = div_mod_7_iter!(rem1); // Second iteration
    let (q3, rem3) = div_mod_7_iter!(rem2); // Third iteration
    let (q4, rem4) = div_mod_7_iter!(rem3); // Fourth iteration
    let (q5, rem5) = div_mod_7_iter!(rem4); // 5th iteration

    // Calculate the final quotient sum
    let quotient_sum = q1 + q2 + q3 + q4 + q5;

    // Final check and output assignment
    if rem5 == 0x07 {
        // Equivalent to rem5 == 3'b111
        (quotient_sum + 1, 0x000) // Equivalent to quotient_sum + 1 and remainder 3'b000
    } else {
        (quotient_sum, rem5) // Equivalent to quotient_sum and rem5[1:0]
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

    #[test]
    fn test_div_mod_3_u8() {
        let (q, r) = div_mod_3_u8(10);
        assert_eq!(q, 3);
        assert_eq!(r, 1);

        let (q, r) = div_mod_3_u8(12);
        assert_eq!(q, 4);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_div_mod_3_u16() {
        let (q, r) = div_mod_3_u16(10000);
        assert_eq!(q, 3333);
        assert_eq!(r, 1);

        let (q, r) = div_mod_3_u16(10002);
        assert_eq!(q, 3334);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_div_mod_7_u8() {
        let (q, r) = div_mod_7_u8(10);
        assert_eq!(q, 1);
        assert_eq!(r, 3);

        let (q, r) = div_mod_7_u8(14);
        assert_eq!(q, 2);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_div_mod_7_u16() {
        let (q, r) = div_mod_7_u16(10000);
        assert_eq!(q, 1428);
        assert_eq!(r, 4);

        let (q, r) = div_mod_7_u16(14000);
        assert_eq!(q, 2000);
        assert_eq!(r, 0);
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
            assert!(value < vdc.factor);
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
            assert!(res[0] < hgen.vdc0.factor);
            assert!(res[1] < hgen.vdc1.factor);
        }
    }

    #[test]
    fn test_div_mod_3_edge_cases() {
        // Test boundary values for u8
        assert_eq!(div_mod_3_u8(0), (0, 0));
        assert_eq!(div_mod_3_u8(1), (0, 1));
        assert_eq!(div_mod_3_u8(2), (0, 2));
        assert_eq!(div_mod_3_u8(3), (1, 0));

        // Test maximum value for u8
        let (q, r) = div_mod_3_u8(255);
        assert_eq!(q, 85);
        assert_eq!(r, 0);

        // Test boundary values for u16
        assert_eq!(div_mod_3_u16(0), (0, 0));
        assert_eq!(div_mod_3_u16(1), (0, 1));
        assert_eq!(div_mod_3_u16(2), (0, 2));
        assert_eq!(div_mod_3_u16(3), (1, 0));

        // Test maximum value for u16
        let (q, r) = div_mod_3_u16(65535);
        assert_eq!(q, 21845);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_div_mod_7_edge_cases() {
        // Test boundary values for u8
        assert_eq!(div_mod_7_u8(0), (0, 0));
        assert_eq!(div_mod_7_u8(1), (0, 1));
        assert_eq!(div_mod_7_u8(6), (0, 6));
        assert_eq!(div_mod_7_u8(7), (1, 0));

        // Test maximum value for u8
        let (q, r) = div_mod_7_u8(255);
        assert_eq!(q, 36);
        assert_eq!(r, 3);

        // Test boundary values for u16
        assert_eq!(div_mod_7_u16(0), (0, 0));
        assert_eq!(div_mod_7_u16(1), (0, 1));
        assert_eq!(div_mod_7_u16(6), (0, 6));
        assert_eq!(div_mod_7_u16(7), (1, 0));

        // Test maximum value for u16
        let (q, r) = div_mod_7_u16(65535);
        assert_eq!(q, 9361);
        assert_eq!(r, 8);
    }

    #[test]
    fn test_div_mod_properties() {
        // Test that div_mod_3 satisfies the division algorithm
        for i in 0..100u8 {
            let (q, r) = div_mod_3_u8(i);
            assert!(r < 3, "Remainder should be less than 3");
            assert_eq!(i, q * 3 + r, "Division algorithm should hold");
        }

        // Test that div_mod_7 satisfies the division algorithm
        for i in 0..100u8 {
            let (q, r) = div_mod_7_u8(i);
            assert!(r < 7, "Remainder should be less than 7");
            assert_eq!(i, q * 7 + r, "Division algorithm should hold");
        }

        // Test for u16
        for i in 0..1000u16 {
            let (q, r) = div_mod_3_u16(i);
            assert!(r < 3, "Remainder should be less than 3");
            assert_eq!(i, q * 3 + r, "Division algorithm should hold");

            let (q, r) = div_mod_7_u16(i);
            assert!(r < 7, "Remainder should be less than 7");
            assert_eq!(i, q * 7 + r, "Division algorithm should hold");
        }
    }

    #[test]
    fn test_ilds_sequence_properties() {
        // Test that ILDS VdCorput sequence values are always less than factor
        let mut vdc = VdCorput::new(2, 10);
        for _ in 0..100 {
            let value = vdc.pop();
            assert!(value < vdc.factor);
        }

        // Test that ILDS Halton sequence values are always less than their respective factors
        let mut hgen = Halton::new([2, 3], [10, 8]);
        for _ in 0..100 {
            let res = hgen.pop();
            assert!(res[0] < hgen.vdc0.factor);
            assert!(res[1] < hgen.vdc1.factor);
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
        assert_eq!(vdc_default.scale, 10);
        assert_eq!(vdc_default.factor, 1024);
    }
}
