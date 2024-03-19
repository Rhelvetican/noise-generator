use crate::utils::Mode;
use image::{ImageFormat, Rgba};
use rand::{thread_rng, Rng};

pub(super) struct Image {
    pub mode: Mode,
    pub format: ImageFormat,
    pub resolution: (u32, u32),
    pub alpha: u8,
}

#[derive(Debug)]
pub enum Error {
    // Io(std::io::Error),
    Image(image::ImageError),
}

impl Image {
    pub fn new(mode: Mode, format: ImageFormat, resolution: (u32, u32)) -> Self {
        Image {
            mode,
            format,
            resolution,
            alpha: 255,
        }
    }
    pub fn generate_image(&self, output_dir: &str) -> Result<(), Error> {
        //Initialize image buffer

        let (width, height) = self.resolution;
        let pallette = [
            Rgba([0, 0, 0, self.alpha]),
            Rgba([255, 255, 255, self.alpha]),
        ];
        let mut imgbuf = image::ImageBuffer::new(width, height);
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

        let output_path = format!(
            "{}/output.{}",
            output_dir,
            image_format_to_str(&self.format)
        );

        imgbuf.save(output_path).map_err(Error::Image)?;
        Ok(())
    }
}

fn image_format_to_str(format: &ImageFormat) -> &str {
    match format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpeg",
        ImageFormat::WebP => "webp",
        ImageFormat::Bmp => "bmp",
        &_ => "png",
    }
}
