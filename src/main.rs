use image::{ImageBuffer, Rgb};

use noise_generator::*;

const OUTPUT_LOCATION: &str = "output/out.png";

fn main() {
    println!("Choose your resolution: ");
    println!("1. SD (640x480)");
    println!("2. HD (1280x720)");
    println!("3. FHD (1920x1080)");
    println!("4. QHD (2560x1440)");
    println!("5. UHD (3840x2160)");

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Failed to read line");

    let res = match read_buffer.as_str().trim() {
        "1" => Resolution::SD,
        "2" => Resolution::HD,
        "3" => Resolution::FHD,
        "4" => Resolution::QHD,
        "5" => Resolution::UHD,
        _ => panic!("Invalid input"),
    };
}
