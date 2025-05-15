use clap::Parser;
use cli::CliArgs;
use generator::Generator;
use image::ImageResult;

use rand::rng;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static MIMALLOC: MiMalloc = MiMalloc;

mod ascii;
mod cli;
mod error;
mod generator;
mod palette;

fn main() -> ImageResult<()> {
    let cli = CliArgs::parse();
    let mut rng = rng();

    let mut gen = Generator::new(&mut rng);

    for _ in 0..(cli.quantity) {
        gen.generate(cli.as_image_config())?;
    }

    Ok(())
}
