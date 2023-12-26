pub enum Mode {
    BlackAndWhiteOnly,
    Grayscale,
}

impl Mode {
    pub fn get_pallette(&self) -> Vec<u8> {
        match self {
            Mode::BlackAndWhiteOnly => (0..=1).collect(),
            Mode::Grayscale => (0..=255).collect(),
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