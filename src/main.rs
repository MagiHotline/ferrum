
// ; entrypoint
mod mtleng;
use mtleng::{MTLEngine};

fn main() {
    let mut engine = MTLEngine::new(
        800,
        600,
        "Metal Window",
        mtleng::WindowSize::Windowed);
    engine.run();
}
