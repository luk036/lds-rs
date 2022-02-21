// use ndarray::{array, Array, Array2, Array1};
use interp::interp;
use ndarray::{array, Array1};
// use csaps::CubicSmoothingSpline;
use super::{Sphere, Vdcorput};
use lazy_static::lazy_static;
use ndarray::Dim;

const TWO_PI: f64 = std::f64::consts::TAU;
const PI: f64 = std::f64::consts::PI;
const HALF_PI: f64 = PI / 2.0;

struct Sp3Table {
    x: Vec<f64>,
    t: Vec<f64>,
}

impl Sp3Table {
    fn new() -> Self {
        let x_ = Array1::linspace(0.0, PI, 300);
        let t_ = 0.5 * (&x_ - &x_.mapv(f64::sin) - &x_.mapv(f64::cos));
        Sp3Table {
            x: x_.to_vec(),
            t: t_.to_vec(),
        }
    }

    fn evaluate(&self, ti: f64) -> f64 {
        interp(&self.t, &self.x, ti)
    }
}

lazy_static! {
    static ref SP3: Sp3Table = Sp3Table::new();
}

// const x: Array1<f64> = Array1::linspace(0.0, PI, 300);
// const t: Array1<f64> = 0.5 * (&x - &x.mapv(f64::sin) - &x.mapv(f64::cos));
// const sp3: CubicSmoothingSpline<f64, Ix1> =
//            CubicSmoothingSpline::new(&t, &x).make().unwrap();

/** Generate Sphere-3 Halton sequence */
pub struct Sphere3 {
    vdc: Vdcorput,
    sphere2: Sphere,
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
        }
    }

    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }

    /**
     * @brief
     *
     * @return Vec<f64>
     */
    pub fn pop(&mut self) -> [f64; 4] {
        let ti = HALF_PI * self.vdc.pop(); // map to [0, pi/2];
                                           // let tiwrap = array![ti];
        let xi = SP3.evaluate(ti);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let [s0, s1, s2] = self.sphere2.pop();
        [sinxi * s0, sinxi * s1, sinxi * s2, cosxi]
    }
}

/**
 * @brief Cylin2 sequence generator
 *
 */
pub struct Cylin2 {
    vdc: Vdcorput,
}

impl Cylin2 {
    pub fn new(base: usize) -> Self {
        Cylin2 {
            vdc: Vdcorput::new(base),
        }
    }

    pub fn pop(&mut self) -> Vec<f64> {
        // let two_pi = 2.0 * (-1.0 as f64).acos(); // ???
        let theta = self.vdc.pop() * TWO_PI; // map to [0, 2*pi];
        vec![theta.sin(), theta.cos()]
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
    }
}

enum CylinVariant {
    ForN(Box<CylinN>),
    For2(Box<Cylin2>),
}

/** Generate using cylindrical coordinate method */
pub struct CylinN {
    vdc: Vdcorput,
    c_gen: CylinVariant,
}

impl CylinN {
    /**
     * @brief Construct a new cylin n::cylin n object
     *
     * @param n
     * @param base
     */
    pub fn new(base: &[usize]) -> Self {
        let n = base.len();
        assert!(n >= 2);
        let c_gen = if n == 2 {
            CylinVariant::For2(Box::<Cylin2>::new(Cylin2::new(base[1])))
        } else {
            CylinVariant::ForN(Box::<CylinN>::new(CylinN::new(&base[1..])))
        };
        CylinN {
            vdc: Vdcorput::new(base[0]),
            c_gen
        }
    }

    /**
     * @brief
     *
     * @return Vec<f64>
     */
    pub fn pop(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0;  // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();

        // ???
        let mut res = vec![];
        match &mut self.c_gen {
            CylinVariant::ForN(gen_n) => { res = gen_n.pop() },
            CylinVariant::For2(gen_2) => { res = gen_2.pop() },
        }

        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }
}

