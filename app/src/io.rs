use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// linux: /home/alice/.local/share/tui_bricks/
/// macos: /Users/Alice/Library/Application Support/com.simaflux.tui_bricks/database.yml
/// windows: C:\Users\Alice\AppData\Roaming\simaflux\tui_bricks\data\database.yml
#[cfg(not(debug_assertions))]
pub fn get_storage_dir() -> Result<std::path::PathBuf> {
    let path = directories::ProjectDirs::from("com", "simaflux", "tui_bricks")
        .map(|dir| dir.data_dir().to_path_buf())
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no valid home directory found using the projectdirs crate, can't use/create local storage dir",
        ))?;

    let create_dir = fs::create_dir_all(&path);
    match create_dir {
        Ok(()) => Ok(path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(path),
            _ => Err(e.into()),
        },
    }
}

/// linux: /home/alice/.config/tui_bricks/
/// macos: /Users/Alice/Library/Caches/com.simaflux.tui_bricks
/// windows: C:\Users\Alice\AppData\Local\simaflux\tui_bricks\cache
#[cfg(not(debug_assertions))]
pub fn get_config_dir() -> Result<std::path::PathBuf> {
    let path = directories::ProjectDirs::from("com", "simaflux", "tui_bricks")
        .map(|dir| dir.config_dir().to_path_buf())
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no valid home directory found using the projectdirs crate, can't use/create config dir",
        ))?;

    let create_dir = fs::create_dir_all(&path);
    match create_dir {
        Ok(()) => Ok(path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(path),
            _ => Err(e.into()),
        },
    }
}

pub fn read_contents_from_path<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let file = fs::File::open(path)?;
    let contents = serde_yaml::from_reader(file)?;
    Ok(contents)
}

pub fn write_contents_to_path<P, T>(path: P, contents: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let file = fs::File::create(path)?;
    serde_yaml::to_writer(file, contents)?;
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

        write_contents_to_path(&file_path, &test).unwrap();
        let yaml = read_contents_from_path(&file_path).unwrap();

        fs::remove_file(file_path).unwrap();

        assert_eq!(test, yaml);
    }
}
