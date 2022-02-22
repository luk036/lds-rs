// use ndarray::{array, Array, Array2, Array1};
// use interp::interp;
// use ndarray::Array1;
// use csaps::CubicSmoothingSpline;
// use ndarray::Dim;
use super::{Circle, Vdcorput};

enum CylinVariant {
    For2(Box<Circle>),
    ForN(Box<CylinN>),
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
            CylinVariant::For2(Box::<Circle>::new(Circle::new(base[1])))
        } else {
            CylinVariant::ForN(Box::<CylinN>::new(CylinN::new(&base[1..])))
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
    pub fn pop(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();

        // ???
        let mut res = match &mut self.c_gen {
            CylinVariant::For2(gen_2) => gen_2.pop().to_vec(),
            CylinVariant::ForN(gen_n) => gen_n.pop(),
        };
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }
}
