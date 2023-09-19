use std::{ops::Deref, path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;

pub static DIRECTORIES: LazyLock<ComicProjDirs> = LazyLock::new(initialize);

fn initialize() -> ComicProjDirs {
    directories::ProjectDirs::from("org", "ang", "comic-leser")
        .map(ComicProjDirs)
        .expect("No HOME directory found on OS")
}

pub struct ComicProjDirs(ProjectDirs);

impl ComicProjDirs {
    pub fn config_file_path(&self) -> PathBuf {
        self.config_dir().join("config.json")
    }
}

impl Deref for ComicProjDirs {
    type Target = ProjectDirs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
