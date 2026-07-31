//! Memory usage profiling for lds-gen generators.
//!
//! Prints stack sizes (via size_of) and heap usage (via dhat).
//!
//! Run:
//!   cargo run --example profile_memory
//!
//! dhat writes `dhat-heap.json` at exit; view with:
//!   cargo install dhat-view  &&  dhat-view dhat-heap.json

use std::mem::size_of;

use dhat::{HeapStats, Profiler};
use lds_rs::sphere_n::{Sphere3, SphereGen, SphereN};
use lds_rs::{Circle, Disk, Halton, HaltonN, Sphere, Sphere3Hopf, VdCorput};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = Profiler::new_heap();

    println!("=== Stack size per struct (bytes) ===\n");

    println!("  VdCorput:    {:>4}", size_of::<VdCorput>());
    println!("  Halton:      {:>4}", size_of::<Halton>());
    println!("  Circle:      {:>4}", size_of::<Circle>());
    println!("  Disk:        {:>4}", size_of::<Disk>());
    println!("  Sphere:      {:>4}", size_of::<Sphere>());
    println!("  Sphere3Hopf: {:>4}", size_of::<Sphere3Hopf>());
    println!("  HaltonN:     {:>4}", size_of::<HaltonN>());
    println!("  Sphere3:     {:>4}", size_of::<Sphere3>());
    println!("  SphereN:     {:>4}", size_of::<SphereN>());
    println!();

    // ------------------------------------------------------------------
    // Heap: exercise all the generators and observe allocations
    // ------------------------------------------------------------------
    println!("=== Generating samples (heap actively tracked) ===\n");

    let mut vgen = VdCorput::new(2);
    vgen.reseed(0);
    for _ in 0..1000 {
        let _ = vgen.pop();
    }

    let mut hgen = Halton::new([2, 3]);
    hgen.reseed(0);
    for _ in 0..500 {
        let _ = hgen.pop();
    }

    let mut cgen = Circle::new(2);
    cgen.reseed(0);
    for _ in 0..500 {
        let _ = cgen.pop();
    }

    let mut dgen = Disk::new([2, 3]);
    dgen.reseed(0);
    for _ in 0..500 {
        let _ = dgen.pop();
    }

    let mut sgen = Sphere::new([2, 3]);
    sgen.reseed(0);
    for _ in 0..500 {
        let _ = sgen.pop();
    }

    let mut h3gen = Sphere3Hopf::new([2, 3, 5]);
    h3gen.reseed(0);
    for _ in 0..500 {
        let _ = h3gen.pop();
    }

    // Sphere3 — exercise both instances to verify shared tables
    let mut sp3_1: Box<dyn SphereGen> = Box::new(Sphere3::new(&[2, 3, 5]));
    sp3_1.reseed(0);
    let mut sp3_2: Box<dyn SphereGen> = Box::new(Sphere3::new(&[5, 7, 11]));
    sp3_2.reseed(0);
    for _ in 0..500 {
        let _ = sp3_1.pop();
        let _ = sp3_2.pop();
    }

    // SphereN with 3 levels (n=3) — verify Arc sharing between instances
    let mut spn_1: Box<dyn SphereGen> = Box::new(SphereN::new(&[2, 3, 5, 7]));
    spn_1.reseed(0);
    let mut spn_2: Box<dyn SphereGen> = Box::new(SphereN::new(&[5, 7, 11, 13]));
    spn_2.reseed(0);
    for _ in 0..500 {
        let _ = spn_1.pop();
        let _ = spn_2.pop();
    }

    // HaltonN
    let bases = &[2, 3, 5, 7, 11, 13];
    let mut hn = HaltonN::new(bases);
    hn.reseed(0);
    for _ in 0..500 {
        let _ = hn.pop();
    }

    let stats = HeapStats::get();
    println!("=== dhat HeapStats ===\n");
    println!(
        "  Total bytes allocated (cumulative): {:>10}",
        stats.total_bytes
    );
    println!(
        "  Total blocks allocated (cumulative): {:>10}",
        stats.total_blocks
    );
    println!(
        "  Max bytes live at any point:        {:>10}",
        stats.max_bytes
    );
    println!(
        "  Current bytes in use:               {:>10}",
        stats.curr_bytes
    );
    println!(
        "  Current blocks in use:              {:>10}",
        stats.curr_blocks
    );
    println!();
    println!("dhat-heap.json written at exit. View with `dhat-view dhat-heap.json`.");
}
