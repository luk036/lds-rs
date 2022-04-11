// #![feature(unboxed_closures)]

const TWO_PI: f64 = std::f64::consts::TAU;

fn vdc(mut k: usize, base: usize) -> f64 {
    let mut vdc = 0.0;
    let mut denom = 1.0;
    while k != 0 {
        denom *= base as f64;
        let remainder = k % base;
        k /= base;
        vdc += (remainder as f64) / denom;
    }
    vdc
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
}

impl Vdcorput {
    pub fn new(base: usize) -> Self {
        Vdcorput { count: 0, base }
    }

    pub fn pop(&mut self) -> f64 {
        self.count += 1;
        vdc(self.count, self.base)
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
/// use lds_rs::Halton;
///
/// let mut hgen = Halton::new(&[2, 3]);
/// hgen.reseed(10);
/// let result = hgen.pop();
///
/// assert_eq!(result[0], 0.8125);
/// ```
pub struct Halton {
    vdc0: Vdcorput,
    vdc1: Vdcorput,
}

impl Halton {
    pub fn new(base: &[usize]) -> Self {
        Halton {
            vdc0: Vdcorput::new(base[0]),
            vdc1: Vdcorput::new(base[1]),
        }
    }

    pub fn pop(&mut self) -> [f64; 2] {
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

/// Circle sequence generator
///
/// # Examples
///
/// ```
/// use lds_rs::Circle;
///
/// let mut cgen = Circle::new(2);
/// cgen.reseed(1);
/// let result = cgen.pop();
///
/// assert_eq!(result[0], 1.0);
/// ```
pub struct Circle {
    vdc: Vdcorput,
}

impl Circle {
    pub fn new(base: usize) -> Self {
        Circle {
            vdc: Vdcorput::new(base),
        }
    }

    pub fn pop(&mut self) -> [f64; 2] {
        // let two_pi = 2.0 * (-1.0 as f64).acos(); // ???
        let theta = self.vdc.pop() * TWO_PI; // map to [0, 2*pi];
        [theta.sin(), theta.cos()]
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
    }
}

/// Sphere sequence generator
///
/// # Examples
///
/// ```
/// use lds_rs::Sphere;
///
/// let mut sgen = Sphere::new(&[2, 3]);
/// sgen.reseed(1);
/// let result = sgen.pop();
///
/// assert_eq!(result[2], -0.5);
/// ```
pub struct Sphere {
    vdc: Vdcorput,
    cirgen: Circle,
}

impl Sphere {
    pub fn new(base: &[usize]) -> Self {
        Sphere {
            vdc: Vdcorput::new(base[0]),
            cirgen: Circle::new(base[1]),
        }
    }

    pub fn pop(&mut self) -> [f64; 3] {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let [c, s] = self.cirgen.pop();
        [sinphi * c, sinphi * s, cosphi]
    }

    /**
     * @brief
     *
     * @param seed
     */
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.cirgen.reseed(seed);
        self.vdc.reseed(seed);
    }
}

/// S(3) sequence generator by Hopf coordinates
///
/// # Examples
///
/// ```
/// use lds_rs::Sphere3Hopf;
///
/// let mut sgen = Sphere3Hopf::new(&[2, 3, 5]);
/// sgen.reseed(0);
/// let result = sgen.pop();
///
/// assert_eq!(result[2], 0.4472135954999573);
/// ```
pub struct Sphere3Hopf {
    vdc0: Vdcorput,
    vdc1: Vdcorput,
    vdc2: Vdcorput,
}

impl Sphere3Hopf {
    pub fn new(base: &[usize]) -> Self {
        Sphere3Hopf {
            vdc0: Vdcorput::new(base[0]),
            vdc1: Vdcorput::new(base[1]),
            vdc2: Vdcorput::new(base[2]),
        }
    }

    pub fn pop(&mut self) -> [f64; 4] {
        let phi = self.vdc0.pop() * TWO_PI; // map to [0, 2*pi];
        let psy = self.vdc1.pop() * TWO_PI; // map to [0, 2*pi];
        let vd = self.vdc2.pop();
        let cos_eta = vd.sqrt();
        let sin_eta = (1.0 - vd).sqrt();
        [
            cos_eta * psy.cos(),
            cos_eta * psy.sin(),
            sin_eta * (phi + psy).cos(),
            sin_eta * (phi + psy).sin(),
        ]
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
        self.vdc2.reseed(seed);
    }
}
