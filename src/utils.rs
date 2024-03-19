use image::{ImageFormat, Rgba};
use rand::{thread_rng, Rng};
use std::{
    fs::{write, DirBuilder},
    path::Path,
};

#[derive(PartialEq)]
pub enum Mode {
    BlackAndWhiteOnly,
    Grayscale,
    Rainbow,
    Red,
    Green,
    Blue,
}

impl Mode {
    pub fn get_pallette(&self) -> Vec<Rgba<u8>> {
        vec![Rgba([0, 0, 0, 255]), Rgba([255, 255, 255, 255])]
    }
    pub fn from_str(mode: &str) -> Self {
        match mode {
            "BlackAndWhiteOnly" => Mode::BlackAndWhiteOnly,
            "Grayscale" => Mode::Grayscale,
            "Rainbow" => Mode::Rainbow,
            "Red" => Mode::Red,
            "Green" => Mode::Green,
            "Blue" => Mode::Blue,
            _ => Mode::BlackAndWhiteOnly,
        }
    }
}

pub struct Image {
    pub mode: Mode,
    pub resolution: (u32, u32),
    pub format: ImageFormat,
    pub alpha: u8,
}

impl Image {
    pub fn new(mode: Mode, resolution: (u32, u32), format: ImageFormat, alpha: u8) -> Self {
        Self {
            mode,
            resolution,
            format,
            alpha,
        }
    }
    pub fn generate_image(self, output_path: &str) {
        //Initialize image buffer

        let (width, height) = self.resolution;
        let mut imgbuf = image::ImageBuffer::new(width, height);
        let pallette = self.mode.get_pallette();
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = pallette[(x + y) as usize % pallette.len()];
        }

        //Generate image

        for (_x, _y, pix) in imgbuf.enumerate_pixels_mut() {
            match self.mode {
                Mode::BlackAndWhiteOnly => {
                    *pix = match thread_rng().gen_bool(0.5f64) {
                        true => Rgba([0, 0, 0, self.alpha]),
                        false => Rgba([255, 255, 255, self.alpha]),
                    }
                }
                Mode::Grayscale => {
                    let r = thread_rng().gen_range(0..=255);
                    *pix = Rgba([r, r, r, self.alpha]);
                }
                Mode::Rainbow => {
                    let r = thread_rng().gen_range(0..=255);
                    let g = thread_rng().gen_range(0..=255);
                    let b = thread_rng().gen_range(0..=255);
                    *pix = Rgba([r, g, b, self.alpha]);
                }
                Mode::Red => {
                    let r = thread_rng().gen_range(0..=255);
                    *pix = Rgba([r, 0, 0, self.alpha]);
                }
                Mode::Green => {
                    let g = thread_rng().gen_range(0..=255);
                    *pix = Rgba([0, g, 0, self.alpha]);
                }
                Mode::Blue => {
                    let b = thread_rng().gen_range(0..=255);
                    *pix = Rgba([0, 0, b, self.alpha]);
                }
            }
        }

        //Save image

        imgbuf.save(output_path).expect("Failed to save image.");
    }
}

pub fn config_init() {
    const DIR_LIST: [&str; 2] = [".config", "output"];
    for dir in DIR_LIST.iter() {
        init_dir(dir);
    }
    if !Path::new(".config/config.json").exists() {
        let default_config = r#"{
            "mode": "BlackAndWhiteOnly",
            "format": "png",
            "resolution": {
                "width": 3840,
                "height": 2160
            },
        }"#;
        write(".config/config.json", default_config).expect("Failed to create default config file");
    }
}

fn init_dir(dir: &str) {
    if !Path::new(dir).exists() {
        DirBuilder::new()
            .recursive(true)
            .create(dir)
            .expect("Failed to create output directory");
    }
}

pub fn img_fmt_from_str(fmt: &str) -> ImageFormat {
    match fmt {
        "png" => ImageFormat::Png,
        "bmp" => ImageFormat::Bmp,
        "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        _ => {
            println!("Invalid input, defaulting to PNG");
            ImageFormat::Png
        }
    }
}
