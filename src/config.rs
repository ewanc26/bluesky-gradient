use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sky_colours: HashMap<String, Vec<u8>>,
    pub name: String,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let config_data = fs::read_to_string(path)
            .context("Failed to read config file")?;
        serde_json::from_str(&config_data)
            .context("Failed to parse config JSON")
    }
}

pub fn get_available_folder(base_folder: &str) -> PathBuf {
    let mut counter = 1;
    let mut folder = PathBuf::from(base_folder);
    while folder.exists() {
        folder = PathBuf::from(format!("{}_{}", base_folder, counter));
        counter += 1;
    }
    folder
}
