#[cfg(not(debug_assertions))]
use directories::ProjectDirs;

use std::path::PathBuf;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! crate_root {
    () => {{
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::PathBuf::from(manifest_dir)
    }};
}

pub trait PathExt {
    #[cfg(debug_assertions)]
    fn workspace_root() -> Self;

    fn cache_dir() -> Self;
    fn config_dir() -> Self;
    fn data_dir() -> Self;
}

impl PathExt for PathBuf {
    #[cfg(debug_assertions)]
    fn workspace_root() -> Self {
        let mut dir = std::env::current_dir().unwrap();

        loop {
            let candidate = dir.join("Cargo.toml");
            if candidate.exists() {
                let text = std::fs::read_to_string(&candidate).unwrap();
                if text.contains("[workspace]") {
                    return dir;
                }
            }

            if !dir.pop() {
                panic!("Workspace root not found");
            }
        }
    }

    fn cache_dir() -> Self {
        #[cfg(debug_assertions)]
        {
            let mut path = Self::workspace_root();
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

    fn config_dir() -> Self {
        #[cfg(debug_assertions)]
        {
            let mut path = Self::workspace_root();
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

    fn data_dir() -> Self {
        #[cfg(debug_assertions)]
        {
            let mut path = Self::workspace_root();
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
}
