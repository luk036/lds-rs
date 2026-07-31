use lds_rs::sphere_n::{Sphere3, SphereGen, SphereN};
use lds_rs::{Circle, Disk, Halton, Sphere, Sphere3Hopf, VdCorput};
use std::time::Instant;

macro_rules! bench {
    ($name:expr, $count:expr, $body:expr) => {{
        let start = Instant::now();
        for _ in 0..$count {
            $body;
        }
        let ns = start.elapsed().as_nanos() as f64 / $count as f64;
        println!("  {:<35} {:>8.1} ns/op  ({} iters)", $name, ns, $count);
    }};
}

fn main() {
    println!("=== Rust (lds-rs) — Full Benchmark ===\n");

    // ----- Basic generators -----
    let mut v = VdCorput::new(2);
    v.reseed(0);
    bench!("VdCorput base 2 [f64]", 1_000_000, {
        std::hint::black_box(v.pop());
    });

    let mut h = Halton::new([2, 3]);
    h.reseed(0);
    bench!("Halton [2,3] [f64;2]", 500_000, {
        std::hint::black_box(h.pop());
    });

    let mut c = Circle::new(2);
    c.reseed(0);
    bench!("Circle base 2 [f64;2]", 500_000, {
        std::hint::black_box(c.pop());
    });

    let mut d = Disk::new([2, 3]);
    d.reseed(0);
    bench!("Disk [2,3] [f64;2]", 500_000, {
        std::hint::black_box(d.pop());
    });

    let mut s = Sphere::new([2, 3]);
    s.reseed(0);
    bench!("Sphere [2,3] [f64;3]", 500_000, {
        std::hint::black_box(s.pop());
    });

    let mut h3 = Sphere3Hopf::new([2, 3, 5]);
    h3.reseed(0);
    bench!("Sphere3Hopf [2,3,5] [f64;4]", 500_000, {
        std::hint::black_box(h3.pop());
    });

    // ----- Sphere-N generators (Vec return) -----
    let mut s3 = Sphere3::new(&[2, 3, 5]);
    s3.reseed(0);
    bench!("Sphere3 [2,3,5] Vec<f64>(4)", 50_000, {
        std::hint::black_box(s3.pop());
    });

    let mut sn4 = SphereN::new(&[2, 3, 5, 7]);
    sn4.reseed(0);
    bench!("SphereN [2,3,5,7] Vec<f64>(5)", 50_000, {
        std::hint::black_box(sn4.pop());
    });

    let mut sn5 = SphereN::new(&[2, 3, 5, 7, 11]);
    sn5.reseed(0);
    bench!("SphereN [2,3,5,7,11] Vec<f64>(6)", 50_000, {
        std::hint::black_box(sn5.pop());
    });
}
