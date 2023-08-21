use std::path::PathBuf;

use anyhow::{anyhow, Result};
use directories::ProjectDirs;

fn get_default_database_path() -> Result<PathBuf> {
    ProjectDirs::from("com", "simaflux", "tui_bricks")
        .and_then(|proj_dirs| Some(proj_dirs.data_dir().join("database.yml")))
        .ok_or(anyhow!("Could not find the default database"))
}
