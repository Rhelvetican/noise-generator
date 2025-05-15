use std::io::Read;

use image::{ImageBuffer, ImageFormat, ImageResult};
use rand::rngs::ThreadRng;
use sha2::{Digest, Sha512};

use crate::{
    ascii::display_hex,
    cli::{Format, Mode, Resolution},
    palette::Palette,
};

#[derive(Debug, Clone, Copy)]
pub struct ImageConfig {
    pub res: Resolution,
    pub mode: Mode,
    pub fmt: Format,
}

pub struct Generator<'a> {
    rng: &'a mut ThreadRng,
}

impl<'a> Generator<'a> {
    #[inline]
    pub fn new(rng: &'a mut ThreadRng) -> Self {
        Self { rng }
    }

    pub fn generate(&mut self, cfg: ImageConfig) -> ImageResult<()> {
        let mut buf = ImageBuffer::new(cfg.res.width, cfg.res.height);
        let mut hasher = Sha512::new();

        for (_, _, pix) in buf.enumerate_pixels_mut() {
            *pix = cfg.mode.generate_color(self.rng);
        }

        hasher.update(
            buf.bytes()
                .filter_map(Result::ok)
                .take(8)
                .collect::<Vec<_>>(),
        );
        let hashed = hasher.finalize();

        buf.save_with_format(
            format!(
                "./{}.{}",
                hashed
                    .into_iter()
                    .map(display_hex)
                    .take(8)
                    .collect::<String>(),
                Into::<ImageFormat>::into(cfg.fmt)
                    .extensions_str()
                    .first()
                    .copied()
                    .unwrap_or("png")
            ),
            cfg.fmt.into(),
        )
    }
}
