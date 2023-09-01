use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::io;

#[derive(Serialize, Deserialize)]
pub struct Config {
    db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut db_path = io::get_storage_dir();
        db_path.push("database.yml");
        Self {
            db_path: db_path.to_string_lossy().to_string(),
        }
    }
}

impl Config {
    pub fn get_db_path(self) -> PathBuf {
        self.db_path.into()
    }
}
