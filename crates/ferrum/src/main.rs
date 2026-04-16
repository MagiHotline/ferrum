// ; entrypoint
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

fn main() {
    let device =
        MTLCreateSystemDefaultDevice()
        .expect("Failed to find a Metal device. Are you on a Mac?");

    println!("Successfully initialized Metal");
    println!("Device: {}", device.name());
}
