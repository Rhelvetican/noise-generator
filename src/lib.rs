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