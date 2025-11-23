#[cfg(not(debug_assertions))]
use directories_next::ProjectDirs;

#[cfg(debug_assertions)]
use std::fs;
use std::path::PathBuf;

#[cfg(debug_assertions)]
pub fn workspace_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();

    loop {
        let candidate = dir.join("Cargo.toml");
        if candidate.exists() {
            let text = fs::read_to_string(&candidate).unwrap();
            if text.contains("[workspace]") {
                return dir;
            }
        }

        if !dir.pop() {
            panic!("Workspace root not found");
        }
    }
}

pub fn cache_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let mut path = workspace_root();
        path.push("cache");
        path
    }

    #[cfg(not(debug_assertions))]
    {
        ProjectDirs::from("com", "simaflux", "tui_bricks")
            .unwrap()
            .cache_dir()
            .to_path_buf()
    }
}

pub fn config_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let mut path = workspace_root();
        path.push("config");
        path
    }

    #[cfg(not(debug_assertions))]
    {
        ProjectDirs::from("com", "simaflux", "tui_bricks")
            .unwrap()
            .config_dir()
            .to_path_buf()
    }
}

pub fn data_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let mut path = workspace_root();
        path.push("data");
        path
    }

    #[cfg(not(debug_assertions))]
    {
        ProjectDirs::from("com", "simaflux", "tui_bricks")
            .unwrap()
            .data_dir()
            .to_path_buf()
    }
}
