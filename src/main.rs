mod lib;
use crate::lib::{Circle, Halton, Sphere, Sphere3Hopf, HaltonN, CylinN, Vdcorput};

mod lds_n;
use crate::lds_n::{Sphere3, SphereN};

fn main() {
    let base: [usize; 5] = [2, 3, 5, 7, 11];

    let mut vgen = Vdcorput::new_default();
    for _i in 0..10 {
        println!("{}", vgen.pop());
    }

    let mut cgen = Circle::new(2);
    for _i in 0..10 {
        println!("{:?}", cgen.pop());
    }

    let mut hgen = Halton::new(&base);
    for _i in 0..10 {
        println!("{:?}", hgen.pop());
    }

    let mut sgen = Sphere::new(&base);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }

    let mut s3fgen = Sphere3Hopf::new(&base);
    for _i in 0..10 {
        println!("{:?}", s3fgen.pop());
    }

    let mut sgen = Sphere3::new(&base);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }

    let mut hgen = HaltonN::new(&base);
    for _i in 0..10 {
        println!("{:?}", hgen.pop());
    }

    let mut cgen = CylinN::new(&base);
    for _i in 0..10 {
        println!("{:?}", cgen.pop());
    }

    let mut sgen = SphereN::new(&base);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }
}
