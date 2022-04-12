use crate::lds::{Circle, Sphere, Vdcorput};
use interp::interp;
use lazy_static::lazy_static;
use ndarray::Array1;

const PI: f64 = std::f64::consts::PI;
const HALF_PI: f64 = PI / 2.0;

lazy_static! {
    static ref X: Array1<f64> = Array1::linspace(0.0, PI, 300);
}

struct Gl {
    x: Array1<f64>,
    neg_cosine: Array1<f64>,
    sine: Array1<f64>,
}

lazy_static! {
    static ref GL: Gl = Gl {
        x: Array1::linspace(0.0, PI, 300),
        neg_cosine: -X.mapv(f64::cos),
        sine: X.mapv(f64::sin),
    };
}

pub trait SphereGen {
    // fn new(base: &[usize]) -> Self;
    fn pop_vec(&mut self) -> Vec<f64>;
    fn reseed(&mut self, seed: usize);
    fn get_tp(&self) -> &Array1<f64>;
}

pub struct Sphere3 {
    vdc: Vdcorput,
    sphere2: Sphere,
    tp: Array1<f64>,
}

/** Generate Sphere-3 Halton sequence */
impl Sphere3 {
    /**
     * @brief Construct a new Sphere3 object
     *
     * @param base
     */
    pub fn new(base: &[usize]) -> Self {
        Sphere3 {
            vdc: Vdcorput::new(base[0]),
            sphere2: Sphere::new(&base[1..3]),
            // tp: 0.5 * (X.mapv(|x| x) - SINE.mapv(|x| x) + NEG_COSINE.mapv(|x| x)),
            tp: 0.5 * (&GL.x - &GL.sine * &GL.neg_cosine),
        }
    }

    pub fn pop(&mut self) -> [f64; 4] {
        let ti = HALF_PI * self.vdc.pop(); // map to [0, pi/2];
        let xi = interp(&self.tp.to_vec(), &X.to_vec(), ti);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let [s0, s1, s2] = self.sphere2.pop();
        [sinxi * s0, sinxi * s1, sinxi * s2, cosxi]
    }
}

/// Generate Sphere-3 Halton sequence
///
/// # Examples
///
/// ```
/// use lds_rs::Sphere3;
/// use lds_rs::SphereGen;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = Sphere3::new(&[2, 3, 5]);
/// sgen.reseed(10);
/// for _i in 0..10 {
///     println!("{:?}", sgen.pop());
/// }
/// let res = sgen.pop();
///
/// assert_approx_eq!(res[0], 0.3430622238280562);
/// ```
impl SphereGen for Sphere3 {
    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }

    fn pop_vec(&mut self) -> Vec<f64> {
        self.pop().to_vec()
    }

    fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }
}

/** Generate Sphere-3 Halton sequence */
pub struct NSphere {
    vdc: Vdcorput,
    s_gen: Box<dyn SphereGen>,
    tp: Array1<f64>,
}

// static IntSinPowerTable sp {};
impl NSphere {
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 3);
        let (s_gen, tp_minus2): (Box<dyn SphereGen>, Array1<f64>) = if n == 3 {
            (Box::new(Sphere3::new(&base[1..4])), GL.neg_cosine.clone())
        } else {
            let s_minus1 = NSphere::new(n - 1, &base[1..]);
            let ssn_minus2 = s_minus1.get_tp_minus1().clone();
            (Box::new(NSphere::new(n - 1, &base[1..])), ssn_minus2)
        };
        let tp = (((n - 1) as f64) * tp_minus2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / n as f64;
        NSphere {
            vdc: Vdcorput::new(base[0]),
            s_gen,
            tp,
        }
    }

    pub fn get_tp_minus1(&self) -> &Array1<f64> {
        self.s_gen.get_tp()
    }
}

/// Generate N-Sphere Halton sequence
///
/// # Examples
///
/// ```
/// use lds_rs::NSphere;
/// use lds_rs::SphereGen;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = NSphere::new(4, &[2, 3, 5, 7, 11]);
/// sgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", sgen.pop_vec());
/// }
/// let res = sgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.006903401092767657);
/// ```
impl SphereGen for NSphere {
    #[allow(dead_code)]
    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.s_gen.reseed(seed);
    }

    fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }

    fn pop_vec(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let ti = self.tp[0] + (self.tp[self.tp.len() - 1] - self.tp[0]) * vd; // map to [t0, tm-1];
        let xi = interp(&self.tp.to_vec(), &X.to_vec(), ti);
        let sinphi = xi.sin();
        let mut res = self.s_gen.pop_vec();
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(xi.cos());
        res
    }
}

enum SphereVariant {
    // ForS2(Box<Sphere>),
    ForS3(Box<Sphere3>),
    ForSn(Box<SphereN>),
}

/// Generate N-Sphere Halton sequence
///
/// # Examples
///
/// ```
/// use lds_rs::SphereN;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = SphereN::new(4, &[2, 3, 5, 7, 11]);
/// sgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", sgen.pop_vec());
/// }
/// let res = sgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.006903401092767657);
/// ```
pub struct SphereN {
    vdc: Vdcorput,
    s_gen: SphereVariant,
    tp: Array1<f64>,
}

// static IntSinPowerTable sp {};
impl SphereN {
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 3);
        let (s_gen, tp_minus2) = match n {
            3 => (
                SphereVariant::ForS3(Box::<Sphere3>::new(Sphere3::new(&base[1..4]))),
                GL.neg_cosine.clone(),
            ),
            _ => {
                let s_minus1 = SphereN::new(n - 1, &base[1..]);
                let ssn_minus2 = s_minus1.get_tp_minus1().clone();
                (
                    SphereVariant::ForSn(Box::<SphereN>::new(s_minus1)),
                    ssn_minus2,
                )
            }
        };
        let tp = (((n - 1) as f64) * tp_minus2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / n as f64;

        SphereN {
            vdc: Vdcorput::new(base[0]),
            s_gen,
            tp,
        }
    }

    pub fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }

    pub fn get_tp_minus1(&self) -> &Array1<f64> {
        match &self.s_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            SphereVariant::ForS3(gen_3) => gen_3.get_tp(),
            SphereVariant::ForSn(gen_n) => gen_n.get_tp(),
        }
    }

    pub fn pop_vec(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let ti = self.tp[0] + (self.tp[self.tp.len() - 1] - self.tp[0]) * vd; // map to [t0, tm-1];
        let xi = interp(&self.tp.to_vec(), &X.to_vec(), ti);
        let sinphi = xi.sin();
        let mut res = match &mut self.s_gen {
            SphereVariant::ForS3(gen_3) => gen_3.pop().to_vec(),
            SphereVariant::ForSn(gen_n) => gen_n.pop_vec(),
        };
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(xi.cos());
        res
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        match &mut self.s_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            SphereVariant::ForS3(gen_3) => gen_3.reseed(seed),
            SphereVariant::ForSn(gen_n) => gen_n.reseed(seed),
        }
    }
}

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

enum CylinVariant {
    For2(Box<Circle>),
    ForN(Box<CylinN>),
}

/// Generate N-Sphere using cylindrical coordinate method */
pub struct CylinN {
    vdc: Vdcorput,
    c_gen: CylinVariant,
}

/// Generate N-Sphere using cylindrical coordinate method */
///
/// # Examples
///
/// ```
/// use lds_rs::CylinN;
/// use approx_eq::assert_approx_eq;
///
/// let mut cgen = CylinN::new(5, &[2, 3, 5, 7, 11]);
/// cgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", cgen.pop_vec());
/// }
/// let res = cgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.032662755534715766);
/// ```
impl CylinN {
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 2);
        let c_gen = if n == 2 {
            CylinVariant::For2(Box::<Circle>::new(Circle::new(base[1])))
        } else {
            CylinVariant::ForN(Box::<CylinN>::new(CylinN::new(n - 1, &base[1..])))
        };
        CylinN {
            vdc: Vdcorput::new(base[0]),
            c_gen,
        }
    }

    /**
     * @brief
     *
     * @return Vec<f64>
     */
    pub fn pop_vec(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();

        // ???
        let mut res = match &mut self.c_gen {
            CylinVariant::For2(gen_2) => gen_2.pop().to_vec(),
            CylinVariant::ForN(gen_n) => gen_n.pop_vec(),
        };
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        match &mut self.c_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            CylinVariant::For2(gen_2) => gen_2.reseed(seed),
            CylinVariant::ForN(gen_n) => gen_n.reseed(seed),
        }
    }
}

pub trait Cylind {
    // fn new(base: &[usize]) -> Self;
    fn pop_vec(&mut self) -> Vec<f64>;
    fn reseed(&mut self, seed: usize);
}

impl Cylind for Circle {
    fn pop_vec(&mut self) -> Vec<f64> {
        self.pop().to_vec()
    }

    fn reseed(&mut self, seed: usize) {
        self.reseed(seed);
    }
}

/** Generate using cylindrical coordinate method */
pub struct CylindN {
    vdc: Vdcorput,
    c_gen: Box<dyn Cylind>,
}

/// Generate N-Sphere using cylindrical coordinate method */
///
/// # Examples
///
/// ```
/// use lds_rs::CylindN;
/// use lds_rs::Cylind;
/// use approx_eq::assert_approx_eq;
///
/// let mut cgen = CylindN::new(5, &[2, 3, 5, 7, 11]);
/// cgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", cgen.pop_vec());
/// }
/// let res = cgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.032662755534715766);
/// ```
impl CylindN {
    /**
     * @brief Construct a new cylin n::cylin n object
     *
     * @param n
     * @param base
     */
    #[allow(dead_code)]
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 2);
        let c_gen: Box<dyn Cylind> = if n == 2 {
            Box::new(Circle::new(base[1]))
        } else {
            Box::new(CylindN::new(n - 1, &base[1..]))
        };
        CylindN {
            vdc: Vdcorput::new(base[0]),
            c_gen,
        }
    }
}

impl Cylind for CylindN {
    fn pop_vec(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let mut res = self.c_gen.pop_vec();
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }

    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.c_gen.reseed(seed);
    }
}
