// This example demonstrates env_logger integration
// Run with: cargo run --example env_logger_example --features env_logger

use lds_gen::VdCorput;
use log::info;

fn main() {
    // Initialize env_logger
    // Use RUST_LOG environment variable to control log level
    // e.g., RUST_LOG=info cargo run --example env_logger_example --features env_logger
    env_logger::init();

    info!("Starting LDS sequence generation");

    let mut sequence = VdCorput::new(2);
    let mut sample = Vec::new();
    for _ in 0..5 {
        sample.push(sequence.pop());
    }
    info!("VdCorput base 2 sample: {:?}", sample);
    println!("VdCorput sample 1: {:?}", sample);

    let mut sequence = VdCorput::new(3);
    let mut sample = Vec::new();
    for _ in 0..5 {
        sample.push(sequence.pop());
    }
    info!("VdCorput base 3 sample: {:?}", sample);
    println!("VdCorput sample 2: {:?}", sample);

    info!("LDS sequence generation complete");
}
