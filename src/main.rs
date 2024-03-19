mod ascii;
mod config;
mod generate;
mod utils;

use config::*;
use generate::Image;
use utils::*;

fn main() -> Result<(), generate::Error> {
    config::init();

    println!("{}", ascii::TITLE);

    // Load config...
    let config = Config::get_config();
    let mode = Mode::from_str(&config.mode);
    let format = get_image_format(&config.format);
    let resolution = config.resolution.tuple();

    let image = Image::new(mode, format, resolution);

    image.generate_image("output")
}
