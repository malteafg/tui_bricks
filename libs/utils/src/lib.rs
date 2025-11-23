use std::path::PathBuf;
use std::fs;

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
