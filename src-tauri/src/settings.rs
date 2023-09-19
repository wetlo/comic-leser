use anyhow::Context;
use std::{io::BufWriter, path::PathBuf};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "../src/entities/")]
pub struct Settings {
    pub libraries: Vec<LibraryConfig>,
    pub selected_library: Option<usize>,
    #[ts(skip)]
    pub next_library_id: usize,
}

#[derive(Debug, Serialize, Deserialize, Default, TS, Clone)]
#[ts(export, export_to = "../src/entities/")]
pub struct LibraryConfig {
    pub id: usize,
    pub name: String,
    pub path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            libraries: Vec::new(),
            selected_library: None,
            next_library_id: 1,
        }
    }
}

impl Settings {
    // TODO: make this async?
    pub fn load_from_config() -> anyhow::Result<Self> {
        let project_dirs = directories::ProjectDirs::from("org", "ang", "comic-leser")
            .expect("No HOME directory found on OS");

        let config_path = project_dirs.config_dir().join("config.json");

        // create default config
        if !config_path.exists() {
            std::fs::create_dir_all(project_dirs.config_dir())?;
            let file = std::fs::File::create(&config_path)?;
            let file = BufWriter::new(file);
            serde_json::to_writer(file, &Settings::default())?;
        }

        let config = std::fs::read_to_string(config_path).context("Failed reading the config")?;

        serde_json::from_str(&config).context("config is not in the valid format")
    }

    pub fn library(&'_ self) -> Option<&LibraryConfig> {
        self.selected_library
            .and_then(|sel| self.libraries.iter().find(|l| l.id == sel))
    }
}
