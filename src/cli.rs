use clap::{value_parser, Parser};

pub use format::*;
pub use mode::*;
pub use resolution::*;

use crate::generator::ImageConfig;

#[derive(Debug, Parser, Clone, Copy)]
#[command(about = crate::ascii::TITLE, long_about = crate::ascii::TITLE)]
#[command(version, author)]
pub struct CliArgs {
    #[arg(short, long, default_value_t = Resolution::default())]
    resolution: Resolution,
    #[arg(short, long, value_enum, default_value_t = Mode::default())]
    mode: Mode,
    #[arg(short, long, value_enum, default_value_t = Format::default())]
    format: Format,
    #[arg(short, long, default_value_t = 1, value_parser = value_parser!(u16).range(1..))]
    pub quantity: u16,
}

impl CliArgs {
    #[inline]
    pub fn as_image_config(&self) -> ImageConfig {
        ImageConfig {
            res: self.resolution,
            mode: self.mode,
            fmt: self.format,
        }
    }
}

mod resolution {
    use std::{
        fmt::{Display, Formatter, Result as FmtRes},
        str::FromStr,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Resolution {
        pub width: u32,
        pub height: u32,
    }

    impl Default for Resolution {
        #[inline]
        fn default() -> Self {
            Self {
                width: 1920,
                height: 1080,
            }
        }
    }

    impl FromStr for Resolution {
        type Err = crate::error::ResolutionParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use crate::error::ResolutionParseError;

            let (w, h) = s
                .split_once('x')
                .ok_or(ResolutionParseError::InvalidSyntax)?;

            Ok(Self {
                width: w.parse()?,
                height: h.parse()?,
            })
        }
    }

    impl Display for Resolution {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
            write!(f, "{}x{}", self.width, self.height)
        }
    }
}

mod mode {
    use clap::ValueEnum;
    use std::fmt::{Display, Formatter, Result as FmtRes};

    #[derive(Debug, Clone, Copy, Default, ValueEnum)]
    pub enum Mode {
        #[default]
        BlackAndWhite,
        Grayscale,
        Rainbow,
        Red,
        Green,
        Blue,
    }

    impl Display for Mode {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
            write!(
                f,
                "{}",
                match self {
                    Self::BlackAndWhite => "BlackAndWhite",
                    Self::Grayscale => "Grayscale",
                    Self::Rainbow => "Rainbow",
                    Self::Red => "Red",
                    Self::Green => "Green",
                    Self::Blue => "Blue",
                }
            )
        }
    }
}

mod format {
    use clap::ValueEnum;
    use image::ImageFormat;
    use std::fmt::{Display, Formatter, Result as FmtRes};

    #[derive(Debug, Clone, Copy, Default, ValueEnum)]
    pub enum Format {
        #[default]
        Png,
        Jpeg,
        Bmp,
        Webp,
    }

    impl Display for Format {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
            write!(
                f,
                "{}",
                match self {
                    Self::Png => "png",
                    Self::Jpeg => "jpeg",
                    Self::Bmp => "bmp",
                    Self::Webp => "webp",
                }
            )
        }
    }

    impl From<Format> for ImageFormat {
        fn from(value: Format) -> Self {
            match value {
                Format::Png => Self::Png,
                Format::Jpeg => Self::Jpeg,
                Format::Bmp => Self::Bmp,
                Format::Webp => Self::WebP,
            }
        }
    }
}
