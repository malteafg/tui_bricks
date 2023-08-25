use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::io;

#[derive(Serialize, Deserialize)]
pub struct Config {
    db_path: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut db_path = io::get_storage_dir()?;
        db_path.push("database.yml");
        let db_path = db_path
            .into_os_string()
            .into_string()
            .map_err(|_| Error::OsStringFailed)?;
        Ok(Self { db_path })
    }

    pub fn get_db_path(self) -> PathBuf {
        self.db_path.into()
    }
}
