use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, exists, File},
    io::{BufReader, Read, Write},
};
use toml::{from_str, Serializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub mode: String,
    pub format: String,
    pub resolution: Resolution,
    pub batch: u32,
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
            batch: 1,
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
    pub fn get_config() -> Result<Config> {
        if !exists("./.config/config.toml")? {
            let def = Config::default();

            if !exists("./.config/")? {
                create_dir("./.config/")?
            };

            let mut cfg = File::create("./.config/config.toml")?;

            let mut buf = String::new();
            let ser = Serializer::new(&mut buf);

            def.serialize(ser)?;
            cfg.write_all(buf.as_bytes())?;

            Ok(def)
        } else {
            let cfg = File::open("./.config/config.toml")?;
            let mut reader = BufReader::new(cfg);

            let content = {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;
                buf
            };

            Ok(from_str(&content)?)
        }
    }
}

impl Resolution {
    pub fn tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
