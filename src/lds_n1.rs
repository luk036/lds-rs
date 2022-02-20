use super::{Circle, Sphere, Vdcorput};

const TWO_PI: f64 = std::f64::consts::TAU;

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
    pub fn new(base: &[usize]) -> Self {
        let mut vdcs = vec![];
        for b in base.iter() {
            vdcs.push(Vdcorput::new(*b));
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
    pub fn reseed(&mut self, seed: usize) {
        for vdc in self.vdcs.iter_mut() {
            vdc.reseed(seed);
        }
    }
}

/**
 * @brief CylinN2 sequence generator
 *
 */
pub struct CylinN2 {
    vdc: Vdcorput,
}

impl CylinN2 {
    pub fn new(base: usize) -> Self {
        CylinN2 {
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

/**
 * @brief Sphere sequence generator
 *
 */
pub struct SphereN2 {
    vdc: Vdcorput,
    cirgen: Circle,
}

impl SphereN2 {
    pub fn new(base: &[usize]) -> Self {
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
    pub fn reseed(&mut self, seed: usize) {
        self.cirgen.reseed(seed);
        self.vdc.reseed(seed);
    }
}
