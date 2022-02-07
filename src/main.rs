mod lib;
use crate::lib::lds::{Circle, Halton, Sphere, Sphere3Hopf, Vdcorput};

fn main() {
    let base: [u32; 4] = [2, 3, 5, 7];

    let mut vgen = Vdcorput::new_default();
    for _i in 0..10 {
        println!("{}", vgen.call());
    }

    let mut cgen = Circle::new(2);
    for _i in 0..10 {
        println!("{:?}", cgen.call());
    }

    let mut hgen = Halton::new(&base);
    for _i in 0..10 {
        println!("{:?}", hgen.call());
    }

    let mut sgen = Sphere::new(&base);
    for _i in 0..10 {
        println!("{:?}", sgen.call());
    }

    let mut s3fgen = Sphere3Hopf::new(&base);
    for _i in 0..10 {
        println!("{:?}", s3fgen.call());
    }
}
