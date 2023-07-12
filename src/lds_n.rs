use crate::lds::VdCorput;

/// The HaltonN struct is a generator for the Halton(n) sequence.
/// 
/// Properties:
/// 
/// * `vdcs`: A vector of VdCorput objects.
pub struct HaltonN {
    vdcs: Vec<VdCorput>,
}

/// Halton(n) sequence generator
///
/// # Examples
///
/// ```
/// use lds_rs::HaltonN;
/// use approx_eq::assert_approx_eq;
///
/// let mut hgen = HaltonN::new(5, &[2, 3, 5, 7, 11]);
/// hgen.reseed(10);
/// for _i in 0..10 {
///     println!("{:?}", hgen.pop_vec());
/// }
/// let res = hgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.65625);
impl HaltonN {
    /// Creates a new [`HaltonN`].
    /// 
    /// The `new` function creates a new `HaltonN` struct with a specified number of `VdCorput`
    /// instances.
    /// 
    /// Arguments:
    /// 
    /// * `n`: The `n` parameter represents the number of dimensions in the Halton sequence. It
    /// determines how many different Van der Corput sequences will be generated.
    /// * `base`: The `base` parameter is a slice of `usize` values. It represents the base values for
    /// each dimension of the Halton sequence. Each dimension of the Halton sequence uses a different
    /// base value to generate the sequence.
    /// 
    /// Returns:
    /// 
    /// The `new` function returns a new instance of the `HaltonN` struct.
    pub fn new(n: usize, base: &[usize]) -> Self {
        HaltonN {
            vdcs: (0..n).map(|i| VdCorput::new(base[i])).collect(),
        }
    }

    /// Returns the pop vec of this [`HaltonN`].
    /// 
    /// The `pop_vec` function returns a vector containing the popped values from each `vdc` in the
    /// `HaltonN` struct.
    /// 
    /// Returns:
    /// 
    /// The `pop_vec` function returns a `Vec<f64>`, which is a vector of `f64` values.
    pub fn pop_vec(&mut self) -> Vec<f64> {
        self.vdcs.iter_mut().map(|vdc| vdc.pop()).collect()
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    pub fn reseed(&mut self, seed: usize) {
        for vdc in &mut self.vdcs {
            vdc.reseed(seed);
        }
    }
}

