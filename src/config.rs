use jsonutils::file::{read_json, write_json};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: "BlackAndWhite".to_string(),
            format: "png".to_string(),
            resolution: Resolution::default(),
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
}

impl Config {
    pub fn get_config() -> Config {
        if !Path::new("./.config/config.json").exists() {
            let def = Config::default();
            write_json("./.config/config.json", def).unwrap();
        }
        read_json("./.config/config.json").unwrap_or_default()
    }
}

impl Resolution {
    pub fn tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
