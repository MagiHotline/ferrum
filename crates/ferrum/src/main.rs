
// ; entrypoint
mod mtleng;
use mtleng::{MTLEngine};

fn main() {

    let engine = unsafe { MTLEngine::new() };
    unsafe {
        engine.run();
        engine.cleanup();
    };
}
