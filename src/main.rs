mod ascii;
mod config;
mod generate;
mod utils;

use std::time::Instant;

use anyhow::Result;
use config::*;
use generate::Image;
use utils::*;

fn main() -> Result<()> {
    println!("{}", ascii::TITLE);

    // Load config...
    let inst = Instant::now();
    let config = Config::get_config();
    let mode = Mode::from_str(&config.mode);
    let format = get_image_format(&config.format);
    let resolution = config.resolution.tuple();

    let image = Image::new(mode, format, resolution);

    image.generate_image("output")?;
    let elapsed = inst.elapsed().as_secs_f64();
    println!("Generated image in {:.2}s", elapsed);
    Ok(())
}
