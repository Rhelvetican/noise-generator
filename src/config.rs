use serde::{Deserialize, Serialize};
use std::{
    fs::{write, DirBuilder},
    path::Path,
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
}

fn dir_init(dir: &str) {
    if !Path::new(dir).exists() {
        DirBuilder::new().recursive(true).create(dir).unwrap();
    }
}
