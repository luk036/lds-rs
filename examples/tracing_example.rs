use lds_rs::lds;
use tracing::{span, Level};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    let mut sequence = lds::VdCorput::new(2);
    let mut sample = Vec::new();
    for _ in 0..5 {
        sample.push(sequence.pop());
    }
    println!("VdCorput sample 1: {:?}", sample);

    let mut sequence = lds::VdCorput::new(3);
    let mut sample = Vec::new();
    for _ in 0..5 {
        sample.push(sequence.pop());
    }
    println!("VdCorput sample 2: {:?}", sample);
}