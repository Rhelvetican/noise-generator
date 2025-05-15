use image::Rgba;
use rand::{rngs::ThreadRng, Rng};

use crate::cli::Mode;

/// Represent a valid color palette from which a color can be generated.
pub trait Palette<Color = Rgba<u8>> {
    /// Generate a random color from the palette.
    fn generate_color(&self, rng: &mut ThreadRng) -> Color;
}

impl Palette for Mode {
    fn generate_color(&self, rng: &mut ThreadRng) -> Rgba<u8> {
        match self {
            Mode::BlackAndWhite => {
                if rng.random() {
                    Rgba([0, 0, 0, 255])
                } else {
                    Rgba([255; 4])
                }
            }
            Mode::Grayscale => {
                let r = rng.random();
                Rgba([r, r, r, 255])
            }
            Mode::Rainbow => {
                let r = rng.random();
                let g = rng.random();
                let b = rng.random();
                Rgba([r, g, b, 255])
            }
            Mode::Red => {
                let r = rng.random();
                Rgba([r, 0, 0, 255])
            }
            Mode::Green => {
                let g = rng.random();
                Rgba([0, g, 0, 255])
            }
            Mode::Blue => {
                let b = rng.random();
                Rgba([0, 0, b, 255])
            }
        }
    }
}
