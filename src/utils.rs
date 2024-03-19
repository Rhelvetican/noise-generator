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
