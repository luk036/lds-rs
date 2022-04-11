mod lds;
use crate::lds::{Circle, Halton, Sphere, Sphere3Hopf, Vdcorput};

mod lds_n;
use crate::lds_n::{Cylind, SphereGen}; // Traits
use crate::lds_n::{CylinN, CylindN, HaltonN, Sphere3, SphereN, NSphere};

fn main() {
    let base: [usize; 5] = [2, 3, 5, 7, 11];

    let mut vgen = Vdcorput::new(2);
    vgen.reseed(10);
    for _i in 0..10 {
        println!("{}", vgen.pop());
    }

    let mut cgen = Circle::new(2);
    cgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", cgen.pop());
    }

    let mut hgen = Halton::new(&base);
    hgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", hgen.pop());
    }

    let mut sgen = Sphere::new(&base);
    sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }

    let mut s3fgen = Sphere3Hopf::new(&base);
    s3fgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", s3fgen.pop());
    }

    let mut sgen = Sphere3::new(&base);
    sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }

    let mut hgen = HaltonN::new(&base);
    hgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", hgen.pop_vec());
    }

    let mut cgen = CylinN::new(&base);
    // cgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", cgen.pop_vec());
    }

    let mut cgen = CylindN::new(&base);
    // cgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", cgen.pop_vec());
    }

    let mut sgen = SphereN::new(&base);
    // sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop_vec());
    }

    let mut sgen = NSphere::new(&base);
    // sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop_vec());
    }
}
