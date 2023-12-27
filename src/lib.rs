use image::{ImageFormat, Rgb};
use rand::thread_rng;

pub enum Mode {
    BlackAndWhiteOnly,
    Grayscale,
}

impl Mode {
    pub fn get_pallette(&self) -> Vec<Rgb<u8>> {
        match self {
            Mode::BlackAndWhiteOnly => vec![Rgb([0, 0, 0]), Rgb([255, 255, 255])],
            Mode::Grayscale => {
                let mut pallette = Vec::new();
                for i in 0..=255 {
                    pallette.push(Rgb([i, i, i]));
                }
                pallette
            }
        }
    }
}

pub enum Resolution {
    SD,
    HD,
    FHD,
    QHD,
    UHD,
}

impl Resolution {
    pub fn get_resolution(&self) -> (u32, u32) {
        match self {
            Resolution::SD => (640, 480),
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::QHD => (2560, 1440),
            Resolution::UHD => (3840, 2160),
        }
    }
}

pub struct Image {
    pub mode: Mode,
    pub resolution: Resolution,
    pub format: ImageFormat,
}

impl Image {
    pub fn new(mode: Mode, resolution: Resolution, format: ImageFormat) -> Self {
        Self {
            mode,
            resolution,
            format,
        }
    }
    pub fn generate_image(self, output_path: &str) {
        let (width, height) = self.resolution.get_resolution();
        let mut imgbuf = image::ImageBuffer::new(width, height);
        let pallette = self.mode.get_pallette();
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = pallette[(x + y) as usize % pallette.len()];
        }
        imgbuf.save(output_path).unwrap();
    }
}
