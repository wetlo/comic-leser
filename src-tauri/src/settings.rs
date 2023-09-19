use anyhow::Context;
use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, BufWriter};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::directories::DIRECTORIES;

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
    pub async fn load_from_config() -> anyhow::Result<Self> {
        let config_path = DIRECTORIES.config_file_path();

        // create default config
        if !config_path.exists() {
            Self::default().persist().await?;
        }

        let config = tokio::fs::read_to_string(config_path)
            .await
            .context("Failed reading the config")?;

        serde_json::from_str(&config).context("config is not in the valid format")
    }

    pub async fn persist(&self) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(DIRECTORIES.config_dir()).await?;
        let file = tokio::fs::File::create(DIRECTORIES.config_file_path()).await?;
        let mut file = BufWriter::new(file);
        let content = serde_json::to_vec_pretty(&self)?;

        file.write_all(&content)
            .await
            .context("can't write config to file")?;

        file.flush().await.context("can't write config to file")
    }

    pub fn library(&'_ self) -> Option<&LibraryConfig> {
        self.selected_library
            .and_then(|sel| self.libraries.iter().find(|l| l.id == sel))
    }
}
