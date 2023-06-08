use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../src/entities/")]
pub struct Settings {
    pub comic_path: PathBuf,
}

impl Settings {
    pub fn load_from_config() -> anyhow::Result<Self> {
        let project_dirs = directories::ProjectDirs::from("org", "ang", "comic-leser")
            .expect("No HOME directory found on OS");

        let config_path = project_dirs.config_dir().join("config.json");
        let config = std::fs::read_to_string(config_path)?;

        Ok(serde_json::from_str(&config)?)
    }
}
