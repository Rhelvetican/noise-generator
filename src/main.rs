use image::ImageFormat;
use noise_generator::*;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!();
    println!("Noise Generator Version {}", VERSION);
    println!();

    println!("Choose your resolution: ");
    println!("1. SD (640x480)");
    println!("2. HD (1280x720)");
    println!("3. FHD (1920x1080)");
    println!("4. QHD (2560x1440)");
    println!("5. UHD (3840x2160)");
    println!("6. Custom (input width and height)");

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
        "6" => {
            println!("Enter width: ");
            let mut read_buffer = String::new();
            std::io::stdin()
                .read_line(&mut read_buffer)
                .expect("Failed to read line");
            let width = read_buffer
                .as_str()
                .trim()
                .parse::<u32>()
                .expect("Failed to parse width");

            println!("Enter height: ");
            let mut read_buffer = String::new();
            std::io::stdin()
                .read_line(&mut read_buffer)
                .expect("Failed to read line");
            let height = read_buffer
                .as_str()
                .trim()
                .parse::<u32>()
                .expect("Failed to parse height");

            Resolution::Custom(width, height)
        }
        _ => {
            println!("Invalid input, defaulting to SD");
            Resolution::SD
        }
    };

    println!("Choose your image format: ");
    println!("1. PNG");
    println!("2. BMP");
    println!("3. JPEG");
    println!("4. WEBP");

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Failed to read line");

    let format = match read_buffer.as_str().trim() {
        "1" => ImageFormat::Png,
        "2" => ImageFormat::Bmp,
        "3" => ImageFormat::Jpeg,
        "4" => ImageFormat::WebP,
        _ => {
            println!("Invalid input, defaulting to PNG");
            ImageFormat::Png
        }
    };

    println!("Choose your mode: ");
    println!("1. Black and White");
    println!("2. Grayscale");
    println!("3. Rainbow");
    println!("4. Red");
    println!("5. Green");
    println!("6. Blue");

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Failed to read line");

    let mode = match read_buffer.as_str().trim() {
        "1" => Mode::BlackAndWhiteOnly,
        "2" => Mode::Grayscale,
        "3" => Mode::Rainbow,
        "4" => Mode::Red,
        "5" => Mode::Green,
        "6" => Mode::Blue,
        _ => {
            println!("Invalid input, defaulting to Black and White");
            Mode::BlackAndWhiteOnly
        }
    };

    println!("Enter your alpha level (0 ~ 255): ");

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Failed to read line");

    let alpha = read_buffer
        .as_str()
        .trim()
        .parse::<u8>()
        .expect("Failed to parse alpha");
    
    if !PathBuf::from("output").exists() {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create("output")
            .expect("Failed to create output directory");
    }

    let img_info = Image::new(mode, res, format, alpha);
    match img_info.format {
        ImageFormat::Png => {
            img_info.generate_image("output/out.png");
        }
        ImageFormat::Bmp => {
            img_info.generate_image("output/out.bmp");
        }
        ImageFormat::Jpeg => {
            img_info.generate_image("output/out.jpeg");
        }
        ImageFormat::WebP => {
            img_info.generate_image("output/out.webp");
        }
        _ => {
            println!("Invalid image format, defaulting to PNG");
            img_info.generate_image("output/out.png");
        }
    };
}
