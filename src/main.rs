use image::ImageFormat;
use noise_generator::*;

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
        _ => {
            println!("Invalid input, defaulting to SD");
            Resolution::SD
        }
    };

    println!("Choose your image format: ");
    println!("1. PNG");
    // println!("2. BMP");
    // println!("3. JPEG");

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Failed to read line");

    let format = match read_buffer.as_str().trim() {
        "1" => ImageFormat::Png,
        // "2" => ImageFormat::Bmp,
        // "3" => ImageFormat::Jpeg,
        _ => {
            println!("Invalid input, defaulting to PNG");
            ImageFormat::Png
        }
    };

    println!("Choose your mode: ");
    println!("1. Black and White");
    println!("2. Grayscale");

    let mut read_buffer = String::new();
    std::io::stdin()
    .read_line(&mut read_buffer)
    .expect("Failed to read line");

    let mode = match read_buffer.as_str().trim() {
        "1" => Mode::BlackAndWhiteOnly,
        "2" => Mode::Grayscale,
        _ => {
            println!("Invalid input, defaulting to Black and White");
            Mode::BlackAndWhiteOnly
        }
    };

    let img_info = Image::new(mode, res, format);
    img_info.generate_image("output/out.png");
}
