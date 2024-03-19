use image::ImageFormat;

pub enum Mode {
    BlackAndWhiteOnly,
    Grayscale,
    Rainbow,
    Red,
    Green,
    Blue,
}

impl Mode {
    pub fn from_str(mode: &str) -> Mode {
        match mode {
            "BlackAndWhiteOnly" => Mode::BlackAndWhiteOnly,
            "Grayscale" => Mode::Grayscale,
            "Rainbow" => Mode::Rainbow,
            "Red" => Mode::Red,
            "Green" => Mode::Green,
            "Blue" => Mode::Blue,
            // Default to BlackAndWhiteOnly
            _ => Mode::BlackAndWhiteOnly,
        }
    }
}

pub fn get_image_format(format: &str) -> ImageFormat {
    match format {
        "png" => ImageFormat::Png,
        "jpeg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        "webp" => ImageFormat::WebP,
        // Default to PNG
        _ => ImageFormat::Png,
    }
}
