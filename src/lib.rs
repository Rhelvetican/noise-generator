use image::{ImageFormat, Rgba};
use rand::{thread_rng, Rng};

#[derive(PartialEq)]
pub enum Mode {
    BlackAndWhiteOnly,
    Grayscale,
    Rainbow,
    Red,
    Green,
    Blue,
    GaussianNoise,
    PerlinNoise,
    ExponentNoise,
}

impl Mode {
    pub fn get_pallette(&self) -> Vec<Rgba<u8>> {
        vec![Rgba([0, 0, 0, 255]), Rgba([255, 255, 255, 255])]
    }
}

pub enum Resolution {
    SD,
    HD,
    FHD,
    QHD,
    UHD,
    Custom(u32, u32),
}

impl Resolution {
    pub fn get_resolution(&self) -> (u32, u32) {
        match self {
            Resolution::SD => (640, 480),
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::QHD => (2560, 1440),
            Resolution::UHD => (3840, 2160),
            Resolution::Custom(width, height) => (*width, *height),
        }
    }
}

pub struct Image {
    pub mode: Mode,
    pub resolution: Resolution,
    pub format: ImageFormat,
    pub alpha: u8,
}

impl Image {
    pub fn new(mode: Mode, resolution: Resolution, format: ImageFormat, alpha: u8) -> Self {
        Self {
            mode,
            resolution,
            format,
            alpha,
        }
    }
    pub fn generate_image(self, output_path: &str) {
        //Initialize image buffer

        let (width, height) = self.resolution.get_resolution();
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
