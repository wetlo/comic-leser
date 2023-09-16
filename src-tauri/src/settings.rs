use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Default, Clone)]
#[ts(export, export_to = "../src/entities/")]
pub struct Settings {
    pub libraries: Vec<LibraryConfig>,
    pub selected_library: usize,
    #[ts(skip)]
    pub next_library_id: usize,
}

#[derive(Debug, Serialize, Deserialize, TS, Default, Clone)]
#[ts(export, export_to = "../src/entities/")]
pub struct LibraryConfig {
    pub id: usize,
    pub name: String,
    pub path: PathBuf,
}

impl Settings {
    pub fn load_from_config() -> anyhow::Result<Self> {
        let project_dirs = directories::ProjectDirs::from("org", "ang", "comic-leser")
            .expect("No HOME directory found on OS");

        let config_path = project_dirs.config_dir().join("config.json");
        let config = std::fs::read_to_string(config_path)?;

        Ok(serde_json::from_str(&config)?)
    }

    pub fn library(&'_ self) -> &LibraryConfig {
        &self.libraries[self.selected_library]
    }
}
