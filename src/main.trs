use approx_eq::assert_approx_eq;
use lds_rs::lds::PRIME_TABLE;
use lds_rs::lds::{Circle, Halton, Sphere, Sphere3Hopf, Vdcorput};
use lds_rs::lds_n::{CylinN, CylindN, HaltonN, NSphere, Sphere3, SphereN};
use lds_rs::lds_n::{Cylind, SphereGen}; // Traits

fn main() {
    let base: [usize; 5] = [2, 3, 5, 7, 11];

    let mut vgen = Vdcorput::new(2);
    vgen.reseed(10);
    for _i in 0..10 {
        println!("{}", vgen.pop());
    }
    let res = vgen.pop();
    assert_approx_eq!(res, 0.65625);

    let mut cgen = Circle::new(2);
    cgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", cgen.pop());
    }
    let res = cgen.pop();
    assert_approx_eq!(res[0], -0.8314696123025452);

    let mut hgen = Halton::new(&base);
    hgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", hgen.pop());
    }
    let res = hgen.pop();
    assert_approx_eq!(res[0], 0.65625);

    let mut sgen = Sphere::new(&base);
    sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }
    let res = sgen.pop();
    assert_approx_eq!(res[0], 0.8722297870746605);

    let mut s3fgen = Sphere3Hopf::new(&base);
    s3fgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", s3fgen.pop());
    }
    let res = s3fgen.pop();
    assert_approx_eq!(res[0], 0.23764785962349413);

    let mut sgen = Sphere3::new(&base);
    sgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", sgen.pop());
    }
    let res = sgen.pop();
    assert_approx_eq!(res[0], 0.3430622238280562);

    let mut hgen = HaltonN::new(5, &PRIME_TABLE);
    hgen.reseed(10);
    for _i in 0..10 {
        println!("{:?}", hgen.pop_vec());
    }
    let res = hgen.pop_vec();
    assert_approx_eq!(res[0], 0.65625);

    let mut cgen = CylinN::new(5, &PRIME_TABLE);
    cgen.reseed(0);
    for _i in 0..10 {
        println!("{:?}", cgen.pop_vec());
    }
    let res = cgen.pop_vec();
    assert_approx_eq!(res[0], 0.032662755534715766);

    let mut cgen = CylindN::new(5, &PRIME_TABLE);
    cgen.reseed(0);
    for _i in 0..10 {
        println!("{:?}", cgen.pop_vec());
    }
    let res = cgen.pop_vec();
    assert_approx_eq!(res[0], 0.032662755534715766);

    let mut sgen = SphereN::new(4, &PRIME_TABLE);
    sgen.reseed(0);
    for _i in 0..10 {
        println!("{:?}", sgen.pop_vec());
    }
    let res = sgen.pop_vec();
    assert_approx_eq!(res[0], 0.006903401092767657);

    let mut sgen = NSphere::new(4, &PRIME_TABLE);
    sgen.reseed(0);
    for _i in 0..10 {
        println!("{:?}", sgen.pop_vec());
    }
    let res = sgen.pop_vec();
    assert_approx_eq!(res[0], 0.006903401092767657);
}
