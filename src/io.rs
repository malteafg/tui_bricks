use std::fs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use directories::ProjectDirs;

use crate::data::Database;

/// linux: /home/alice/.local/share/tui_bricks/database.yml
/// macos: /Users/Alice/Library/Application Support/com.simaflux.tui_bricks/database.yml
/// windows: C:\Users\Alice\AppData\Roaming\simaflux\tui_bricks\data\database.yml
pub fn get_default_database_path() -> anyhow::Result<PathBuf> {
    let path = ProjectDirs::from("com", "simaflux", "tui_bricks")
        .map(|dir| dir.data_dir().join("database.yml"))
        .ok_or(anyhow!("Could not find the default database"))?;
    let create_dir = fs::create_dir_all(&path);
    match create_dir {
        Ok(()) => Ok(path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(path),
            _ => Err(anyhow!("Could not create directory for tui_bricks data")),
        },
    }
}

pub fn read_database_from_path<P>(path: P) -> anyhow::Result<Database>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(path)?;
    let database = serde_yaml::from_reader(file)?;
    Ok(database)
}

pub fn write_database_to_path<P>(path: P, database: &Database) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let file = fs::File::create(path)?;
    serde_yaml::to_writer(file, database)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::get_test_database;

    #[test]
    fn test_read_write_database() {
        let test = get_test_database();

        // let path = get_default_database_path().unwrap();
        let file_path = "output.txt";

        write_database_to_path(&file_path, &test).unwrap();
        let yaml = read_database_from_path(&file_path).unwrap();

        fs::remove_file(file_path).unwrap();

        assert_eq!(test, yaml);
    }
}
