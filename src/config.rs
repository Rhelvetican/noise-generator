use crate::utils::{Image, Mode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Config {
    pub address: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}
