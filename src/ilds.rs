// #![feature(unboxed_closures)]

const fn vdc_i(mut k: usize, base: usize, scale: u32) -> usize {
    let mut res = 0;
    let mut factor = base.pow(scale);
    while k != 0 {
        let remainder = k % base;
        factor /= base;
        k /= base;
        res += remainder * factor;
    }
    res
}

/// Van der Corput sequence generator
///
/// # Examples
///
/// ```
/// use lds_rs::VdCorput;
///
/// let mut vgen = VdCorput::new(2);
/// vgen.reseed(10);
/// let result = vgen.pop();
///
/// assert_eq!(result, 0.8125);
/// ```
pub struct VdCorput {
    count: usize,
    base: usize,
    scale: u32,
}

impl VdCorput {
    /// Creates a new [`VdCorput`].
    pub const fn new(base: usize, scale: u32) -> Self {
        VdCorput {
            count: 0,
            base,
            scale,
        }
    }

    /// Returns the pop of this [`VdCorput`].
    ///
    /// # Examples
    ///
    /// ```
    /// use lds_rs::ilds::VdCorput;
    ///
    /// let mut vd_corput = VdCorput::new(2, 10);
    /// assert_eq!(vd_corput.pop(), 512);
    /// ```
    pub fn pop(&mut self) -> usize {
        self.count += 1;
        vdc_i(self.count, self.base, self.scale)
    }

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

/// Halton sequence generator
///
/// # Examples
///
/// ```
/// use lds_rs::ilds::Halton;
///
/// let mut hgen = Halton::new(&[2, 3], &[11, 7]);
/// hgen.reseed(0);
/// let result = hgen.pop();
///
/// assert_eq!(result[0], 1024);
/// ```
pub struct Halton {
    vdc0: VdCorput,
    vdc1: VdCorput,
}

impl Halton {
    /// Creates a new [`Halton`].
    pub fn new(base: &[usize], scale: &[u32]) -> Self {
        Halton {
            vdc0: VdCorput::new(base[0], scale[0]),
            vdc1: VdCorput::new(base[1], scale[1]),
        }
    }

    /// Returns the pop of this [`Halton`].
    pub fn pop(&mut self) -> [usize; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}
