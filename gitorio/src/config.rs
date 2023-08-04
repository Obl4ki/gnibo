use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub save_name: String,
    pub save_dir: String,
    pub steam_path: String,
    pub dry_run: bool,
    pub refresh_interval_seconds: u64,
}

pub fn load_config(path: PathBuf) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    serde_yaml::from_str(&config_str).map_err(|e| eyre!("Failed to parse string: {e}"))
}
