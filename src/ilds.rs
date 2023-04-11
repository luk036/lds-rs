// #![feature(unboxed_closures)]

// const function
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
/// use lds_rs::Vdcorput;
///
/// let mut vgen = Vdcorput::new(2);
/// vgen.reseed(10);
/// let result = vgen.pop();
///
/// assert_eq!(result, 0.8125);
/// ```
pub struct Vdcorput {
    count: usize,
    base: usize,
    scale: u32,
}

impl Vdcorput {
    pub const fn new(base: usize, scale: u32) -> Self {
        Vdcorput {
            count: 0,
            base,
            scale,
        }
    }

    pub fn pop(&mut self) -> usize {
        self.count += 1;
        vdc_i(self.count, self.base, self.scale)
    }

    pub fn reseed(&mut self, seed: usize) {
        self.count = seed;
    }
}

// impl FnOnce<()> for Vdcorput {
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
    vdc0: Vdcorput,
    vdc1: Vdcorput,
}

impl Halton {
    pub fn new(base: &[usize], scale: &[u32]) -> Self {
        Halton {
            vdc0: Vdcorput::new(base[0], scale[0]),
            vdc1: Vdcorput::new(base[1], scale[1]),
        }
    }

    pub fn pop(&mut self) -> [usize; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /**
     * @brief
     *
     * @param seed
     */
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}
