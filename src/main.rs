mod config;
mod utils;

use utils::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!();
    println!("Noise Generator Version {}", VERSION);
    println!();

    config_init();
}
