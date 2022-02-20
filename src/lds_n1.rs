use super::{Vdcorput, Circle, Sphere};

/**
 * @brief Halton(n) sequence generator
 *
 */
pub struct HaltonN {
    vdcs: Vec<Vdcorput>,
}

/**
 * @brief Halton(n) sequence generator
 *
 */
impl HaltonN {
    /**
     * @brief Construct a new halton n object
     *
     * @param n
     * @param base
     */
    pub fn new(base: &[usize]) -> HaltonN {
        let mut vdcs = vec![];
        for b in base.iter() {
            vdcs.push(Vdcorput::new(b));
        }
        HaltonN { vdcs }
    }

    /**
     * @brief
     *
     * @return let mut
     */
    pub fn pop(&mut self) -> Vec<f64> {
        let mut res = vec![];
        for vdc in self.vdcs.iter_mut() {
            res.push(vdc.pop());
        }
        return res;
    }

    /**
     * @brief
     *
     * @param seed
     */
    pub fn reseed(seed: usize) {
        for vdc in self.vdcs.iter_mut() {
            vdc.reseed(seed);
        }
    }
}

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
    pub fn new(base: &[usize]) -> Sphere3 {
        Sphere3 {
            vdc: Vdcorput::new(base[0]),
            sphere2: Sphere::new(base[1..3]),
        }
    }

    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.Sphere2.reseed(seed);
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
    pub fn new(base: u32) -> Cylin2 {
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
    pub fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
    }
}

/** Generate using cylindrical coordinate method */
pub struct CylinN {
    Vdcorput _vdc;
    std::variant<Box<CylinN>, Box<Cylin2>> _Cgen;
}

/** Generate using cylindrical coordinate method */
impl CylinN {
    /**
     * @brief Construct a new cylin n object
     *
     * @param n dimension
     * @param base sequence base
     */
    CylinN(base: &[usize]);

    /**
     * @brief
     *
     * @return Vec<f64>
     */
    pub fn pop(&mut self) -> Vec<f64>;
}

/**
 * @brief Sphere sequence generator
 *
 */
pub struct SphereN2 {
    vdc: Vdcorput,
    cirgen: Circle,
}

impl SphereN2 {
    pub fn new(base: &[u32]) -> SphereN2 {
        SphereN2 {
            vdc: Vdcorput::new(base[0]),
            cirgen: Circle::new(base[1]),
        }
    }

    pub fn pop(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let [c, s] = self.cirgen.pop();
        vec![sinphi * c, sinphi * s, cosphi]
    }

    /**
     * @brief
     *
     * @param seed
     */
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: u32) {
        self.cirgen.reseed(seed);
        self.vdc.reseed(seed);
    }
}

/** Generate Sphere-3 Halton sequence */
pub struct SphereN {
    Vdcorput _vdc;
    size_t _n;
    std::variant<Box<SphereN>, Box<SphereN2>> _Sgen;
    f64 _range_t;
    f64 _t0;
}

