use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write, DirBuilder},
    path::Path,
    process::exit,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub mode: String,
    pub format: String,
    pub resolution: Resolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Config {
    pub fn get_config() -> Config {
        let config = match read_to_string(".config/config.json") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading config file: {}", e);
                exit(1);
            }
        };
        match serde_json::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error parsing config file: {}", e);
                println!("Using default config");
                Config::default()
            }
        }
    }
}

impl Resolution {
    pub fn tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// Defining default values for the Config and Resolution structs

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: "BlackAndWhite".to_string(),
            format: "png".to_string(),
            resolution: Resolution::default(),
        }
    }
}

pub fn init() {
    const DIR_LIST: [&str; 2] = [".config", "output"];
    for dir in DIR_LIST.iter() {
        dir_init(dir);
    }
    let default_config = r#"
{
    "mode": "BlackAndWhite",
    "format": "png",
    "resolution": {
        "width": 1920,
        "height": 1080
    }
}
    "#;
    if !Path::new(".config/config.json").exists() {
        write(".config/config.json", default_config).unwrap();
    }
}

fn dir_init(dir: &str) {
    if !Path::new(dir).exists() {
        match DirBuilder::new().recursive(true).create(dir) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                exit(1);
            }
        };
    }
}
