
// ; entrypoint
mod mtleng;
use mtleng::{MTLEngine};

fn main() {
    let mut engine = MTLEngine::new();
    engine.run();
}
