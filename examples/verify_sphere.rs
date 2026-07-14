use lds_gen::sphere_n::{Sphere3, SphereGen, SphereN};

fn main() {
    // Sphere3 = 4D (n=3 sphere)
    let mut s3 = Sphere3::new(&[2, 3, 5]);
    s3.reseed(0);
    println!("Sphere3 [2,3,5] seed=0, first 3 points:");
    for i in 0..3 {
        let p = s3.pop();
        print!("  {i}: [");
        for (j, &v) in p.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{v:.16}");
        }
        println!("]");
    }

    // SphereN with 4 bases = 5D (n=4 sphere)
    let mut sn4 = SphereN::new(&[2, 3, 5, 7]);
    sn4.reseed(0);
    println!("\nSphereN [2,3,5,7] -> 5D, seed=0, first 3 points:");
    for i in 0..3 {
        let p = sn4.pop();
        print!("  {i}: [");
        for (j, &v) in p.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{v:.16}");
        }
        println!("]");
    }

    // SphereN with 5 bases = 6D (n=5 sphere)
    let mut sn5 = SphereN::new(&[2, 3, 5, 7, 11]);
    sn5.reseed(0);
    println!("\nSphereN [2,3,5,7,11] -> 6D, seed=0, first 3 points:");
    for i in 0..3 {
        let p = sn5.pop();
        print!("  {i}: [");
        for (j, &v) in p.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{v:.16}");
        }
        println!("]");
    }
}
