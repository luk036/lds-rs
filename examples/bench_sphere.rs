use lds_gen::sphere_n::{Sphere3, SphereGen, SphereN};
use std::time::Instant;

macro_rules! bench {
    ($name:expr, $count:expr, $body:expr) => {{
        let start = Instant::now();
        for _ in 0..$count {
            $body;
        }
        let ns = start.elapsed().as_nanos() as f64 / $count as f64;
        println!("  {:<30} {:>8.1} ns/op  ({} iters)", $name, ns, $count);
    }};
}

fn main() {
    println!("=== Sphere-N Benchmarks (Rust) ===");

    let mut s3 = Sphere3::new(&[2, 3, 5]);
    s3.reseed(0);
    bench!("Sphere3 [2,3,5] 4D", 50000, s3.pop());

    let mut sn4 = SphereN::new(&[2, 3, 5, 7]);
    sn4.reseed(0);
    bench!("SphereN [2,3,5,7] 5D", 50000, sn4.pop());

    let mut sn5 = SphereN::new(&[2, 3, 5, 7, 11]);
    sn5.reseed(0);
    bench!("SphereN [2,3,5,7,11] 6D", 50000, sn5.pop());
}
