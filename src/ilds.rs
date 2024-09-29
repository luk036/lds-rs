// #![feature(unboxed_closures)]
/// Low-Discrepancy Sequence (LDS) Generator (specific for integer output)
///
/// This code implements two low-discrepancy sequence generators: the Van der Corput sequence and the Halton sequence (specific for integer output). These sequences are used to generate evenly distributed points in a space, which can be useful for various applications like sampling, optimization, or numerical integration.
///
/// The code defines three main components: a function called vdc_i, and two classes named VdCorput and Halton.
///
/// The vdc_i function is the core of the Van der Corput sequence generation. It takes an integer k, a base (default 2), and a scale (default 10) as inputs. It converts the number k from the given base to a decimal number, using the specified scale for integer output. This function is used to generate individual elements of the Van der Corput sequence.
///
/// The VdCorput class is a wrapper around the vdc_i function. It keeps track of the current count and allows you to generate successive elements of the Van der Corput sequence by calling its pop method. You can also reset the sequence to a specific starting point using the reseed method.
///
/// The Halton class generates points in a 2-dimensional space using two Van der Corput sequences with different bases. It creates two VdCorput objects internally and uses them to generate pairs of numbers. The pop method of the Halton class returns a list of two integers, representing a point in 2D space.
///
/// The main logic flow in this code is the generation of these low-discrepancy sequences. For the Van der Corput sequence, it works by repeatedly dividing the input number by the base and using the remainders to construct the output number. This process creates a sequence of numbers that are well-distributed between 0 and N (when properly scaled).
///
/// The Halton sequence extends this idea to multiple dimensions by using different bases for each dimension. In this implementation, it generates 2D points by combining two Van der Corput sequences.
///
/// The code doesn't take any direct input from the user. Instead, it provides classes and functions that can be used in other programs to generate these sequences. The output of these generators are individual numbers (for Van der Corput) or pairs of numbers (for Halton) that form the respective sequences.
///
/// This code is particularly useful for applications that need well-distributed random-like numbers, but with more uniformity than typical pseudo-random number generators provide. It's a building block that can be used in more complex algorithms and simulations.

/// The function `vdc_i` calculates the van der Corput sequence for a given base and scale.
///
/// Arguments:
///
/// * `k`: The parameter `k` represents the number that we want to convert to variable digit code (VDC)
/// representation.
/// * `base`: The `base` parameter represents the base of the number system being used. It determines
/// the number of unique digits that can be used to represent numbers. For example, in base 10, the
/// digits range from 0 to 9.
/// * `scale`: The `scale` parameter in the `vdc_i` function represents the power to which the `base` is
/// raised. It determines the number of digits in the resulting VDC (Van der Corput) number.
///
/// Returns:
///
/// The function `vdc_i` returns an unsigned integer value of type `usize`.
///
/// # Examples
///
/// ```rust
/// use lds_rs::ilds::vdc_i;
///
/// assert_eq!(vdc_i(10, 2, 2), 1);
/// assert_eq!(vdc_i(10, 2, 3), 2);
/// ```
pub const fn vdc_i(k: usize, base: usize, scale: u32) -> usize {
    let mut res = 0;
    let mut factor = base.pow(scale);
    let mut k = k;
    while k != 0 {
        let remainder = k % base;
        factor /= base;
        k /= base;
        res += remainder * factor;
    }
    res
}

/// The `VdCorput` struct is a generator for Van der Corput sequences.
///
/// Properties:
///
/// * `count`: The `count` property represents the number of elements that have been generated from the
/// Van der Corput sequence so far.
/// * `base`: The `base` property represents the base of the Van der Corput sequence. It determines the
/// distribution of the generated numbers.
/// * `scale`: The `scale` property determines the precision of the generated Van der Corput sequence.
/// It represents the number of bits used to represent the fractional part of the sequence. A higher
/// scale value will result in a more precise sequence, but will also require more memory and
/// computation.
///
/// # Examples
///
/// ```rust
/// use lds_rs::VdCorput;
///
/// let mut vgen = VdCorput::new(2);
/// vgen.reseed(10);
/// let result = vgen.pop();
///
/// assert_eq!(result, 0.8125);
/// ```
#[derive(Debug)]
pub struct VdCorput {
    count: usize,
    base: usize,
    scale: u32,
}

impl VdCorput {
    /// Creates a new [`VdCorput`].
    ///
    /// The `new` function creates a new `VdCorput` struct with the specified base and scale values.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter represents the base of the Van der Corput sequence. It determines the
    /// number of unique values that can be generated by the sequence.
    /// * `scale`: The `scale` parameter in the `new` function is of type `u32`, which stands for unsigned
    /// 32-bit integer. It represents the scale factor used in the `VdCorput` struct.
    ///
    /// Returns:
    ///
    /// The `new` function returns an instance of the `VdCorput` struct.
    pub const fn new(base: usize, scale: u32) -> Self {
        VdCorput {
            count: 0,
            base,
            scale,
        }
    }

    /// Returns the pop of this [`VdCorput`].
    ///
    /// The `pop` method of the [`VdCorput`] struct returns the next element in the Van der Corput
    /// sequence. It increments the `count` property of the struct and uses the `vdc_i` function to
    /// calculate the corresponding Van der Corput value based on the current count, base, and scale. In
    /// the example provided, a `VdCorput` instance is created with a base of 2 and a scale of 10. The
    /// `pop` method is then called, which returns the Van der Corput value for the current count (which
    /// is initially 0). In this case, the returned value is 512.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lds_rs::ilds::VdCorput;
    ///
    /// let mut vd_corput = VdCorput::new(2, 10);
    /// assert_eq!(vd_corput.pop(), 512);
    /// ```
    pub fn pop(&mut self) -> usize {
        self.count += 1;
        vdc_i(self.count, self.base, self.scale)
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    pub fn reseed(&mut self, seed: usize) {
        self.count = seed;
    }
}

// impl FnOnce<()> for VdCorput {
//     type Output = f64;
//     extern "rust-call" fn call_once(self, _arg: ()) -> Self::Output {
//         self.count += 1;
//         vdc(self.count, self.base)
//     }
// }

/// The [`Halton`] struct is a generator for the Halton sequence.
///
/// Properties:
///
/// * `vdc0`: An instance of the VdCorput struct used for generating the first dimension of the Halton
/// sequence.
/// * `vdc1`: The `vdc1` property is an instance of the [`VdCorput`] struct. It is used to generate the
/// Van der Corput sequence for the second dimension of the Halton sequence.
///
/// # Examples
///
/// ```rust
/// use lds_rs::ilds::Halton;
///
/// let mut hgen = Halton::new(&[2, 3], &[11, 7]);
/// hgen.reseed(0);
/// let result = hgen.pop();
///
/// assert_eq!(result[0], 1024);
/// ```
#[derive(Debug)]
pub struct Halton {
    vdc0: VdCorput,
    vdc1: VdCorput,
}

impl Halton {
    /// Creates a new [`Halton`].
    ///
    /// The `new` function creates a new `Halton` struct with specified base and scale values for two
    /// VdCorput sequences.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an array of `usize` values that represents the base for each
    /// dimension of the Halton sequence. Each element in the `base` array corresponds to a dimension in
    /// the sequence.
    /// * `scale`: The `scale` parameter is an array of `u32` values that determine the scale or
    /// precision of the Halton sequence for each dimension. Each element in the `scale` array
    /// corresponds to a dimension in the Halton sequence. The larger the value of `scale`, the more
    /// precise the Halton sequence.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Halton` struct.
    pub fn new(base: &[usize], scale: &[u32]) -> Self {
        Halton {
            vdc0: VdCorput::new(base[0], scale[0]),
            vdc1: VdCorput::new(base[1], scale[1]),
        }
    }

    /// The `pop` function returns an array containing the pop values of two [`Halton`] instances.
    ///
    /// Returns:
    ///
    /// An array of two `usize` values is being returned.
    pub fn pop(&mut self) -> [usize; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdc() {
        let base = 2;
        let scale = 10;
        let k = 10;
        let res = vdc_i(k, base, scale);
        assert_eq!(res, 320);
    }

    #[test]
    fn test_vdcorput() {
        let mut vgen = VdCorput::new(2, 10);
        vgen.reseed(10);
        let result = vgen.pop();
        assert_eq!(result, 832);
    }

    #[test]
    fn test_halton() {
        let mut hgen = Halton::new(&[2, 3], &[11, 7]);
        hgen.reseed(0);
        let result = hgen.pop();
        assert_eq!(result, [1024, 729]);
    }
}
