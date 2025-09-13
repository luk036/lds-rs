pub mod ilds;
pub mod lds;
// pub mod lds_n;

pub use crate::lds::HaltonN;
pub use crate::lds::PRIME_TABLE;
pub use crate::lds::{Circle, Disk, Halton, Sphere, Sphere3Hopf, VdCorput};

#[cfg(test)]
mod tests {
    use super::lds::*;
    // use super::lds_n::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn it_works() {
        let base: [usize; 5] = [2, 3, 5, 7, 11];

        let mut vgen = VdCorput::new(2);
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
        assert_approx_eq!(res[1], -0.8314696123025452);

        let mut dgen = Disk::new(&[2, 3]);
        dgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", dgen.pop());
        }
        let res = dgen.pop();
        assert_approx_eq!(res[0], 0.32102183949750684);

        let mut hgen = Halton::new(&[2, 3]);
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
        assert_approx_eq!(res[1], 0.8722297870746605);

        let mut s3fgen = Sphere3Hopf::new(&base);
        s3fgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", s3fgen.pop());
        }
        let res = s3fgen.pop();
        assert_approx_eq!(res[0], 0.23764785962349413);

        let mut hgen = HaltonN::new(&PRIME_TABLE[0..5]);
        hgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", hgen.pop_vec());
        }
        let res = hgen.pop_vec();
        assert_approx_eq!(res[0], 0.65625);
    }
}
