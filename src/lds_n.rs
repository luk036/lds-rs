use crate::lds::VdCorput;

/**
 * @brief Halton(n) sequence generator
 *
 */
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
    pub fn new(n: usize, base: &[usize]) -> Self {
        HaltonN {
            vdcs: (0..n).map(|i| VdCorput::new(base[i])).collect(),
        }
    }

    /// Returns the pop vec of this [`HaltonN`].
    pub fn pop_vec(&mut self) -> Vec<f64> {
        self.vdcs.iter_mut().map(|vdc| vdc.pop()).collect()
    }

    pub fn reseed(&mut self, seed: usize) {
        for vdc in &mut self.vdcs {
            vdc.reseed(seed);
        }
    }
}

