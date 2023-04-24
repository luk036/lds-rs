use crate::lds::Vdcorput;

/**
 * @brief Halton(n) sequence generator
 *
 */
pub struct HaltonN {
    vdcs: Vec<Vdcorput>,
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
    /**
     * @brief Construct a new halton n object
     *
     * @param n
     * @param base
     */
    pub fn new(n: usize, base: &[usize]) -> Self {
        let mut vdcs = vec![];
        for b in base.iter().take(n) {
            vdcs.push(Vdcorput::new(*b));
        }
        HaltonN { vdcs }
    }

    pub fn pop_vec(&mut self) -> Vec<f64> {
        let mut res = vec![];
        for vdc in self.vdcs.iter_mut() {
            res.push(vdc.pop());
        }
        res
    }

    pub fn reseed(&mut self, seed: usize) {
        for vdc in self.vdcs.iter_mut() {
            vdc.reseed(seed);
        }
    }
}
