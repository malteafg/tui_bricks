use rebrickable_client::ClientDB;
use rebrickable_database_api::{PartId, RebrickableDB};
use utils::{DisplayShortExt, PathExt};

use clap::Parser;

use std::fs;
use std::path::{Path, PathBuf};

static NO_IMAGE: &[u8] = include_bytes!("../assets/no_image.png");

// TODO: use the database so we can also copy images based on name of the part.
// TODO have argument to specify path to images, such that we can enter a tui logo.
#[derive(Parser, Debug)]
pub struct Args {
    /// The id of the part whose image is to be updated.
    part_id: PartId,

    /// The path to copy the file to. This defaults to the cache directory of the os with a folder
    /// called tui_bricks.
    #[arg(long)]
    dst_path: Option<PathBuf>,

    /// The path where images of parts are located.
    #[arg(long)]
    images_path: Option<PathBuf>,
}

fn image_path(base_path: impl AsRef<Path>, file_name: impl AsRef<Path>) -> PathBuf {
    let mut result = base_path.as_ref().to_path_buf();
    result.push(file_name.as_ref());
    result.set_extension("png");
    result
}

fn try_copy_image(
    base_path: impl AsRef<Path>,
    part_id: &PartId,
    dst_path: impl AsRef<Path>,
) -> bool {
    let base_path = base_path.as_ref();
    let path = image_path(base_path, &**part_id);

    if fs::exists(&path).unwrap() {
        fs::copy(&path, &dst_path).unwrap();
        return true;
    }

    if let Some(trimmed) = part_id.trim_id() {
        let path = image_path(&base_path, &*trimmed);
        if fs::exists(&path).unwrap() {
            fs::copy(&path, &dst_path).unwrap();
            return true;
        }
    }

    return false;
}

fn run(mut args: Args) {
    let mut dst_path = match args.dst_path.take() {
        Some(dst_path) => dst_path,
        None => PathBuf::cache_dir(),
    };
    if dst_path.extension().is_none() {
        dst_path.push("displayed_image.png");
    }
    if let Some(parent) = dst_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let part_id = args.part_id;

    let database = ClientDB::new();
    let part = match &database {
        Ok(database) => {
            let part = database.part_from_id(&part_id);
            part.iter().for_each(|p| println!("{}", p.short()));
            part
        }
        Err(_) => {
            println!("{}", part_id);
            None
        }
    };

    let base_path = match args.images_path.take() {
        Some(base_path) => base_path,
        None => {
            let mut base_path = PathBuf::data_dir();
            base_path.push("part_images");
            base_path
        }
    };

    if try_copy_image(&base_path, &part_id, &dst_path) {
        return;
    }

    let Some(part) = part else {
        fs::write(&dst_path, NO_IMAGE).unwrap();
        return;
    };

    for parent_part in part.parent_rels.keys() {
        if try_copy_image(&base_path, &parent_part, &dst_path) {
            return;
        }
    }

    fs::write(&dst_path, NO_IMAGE).unwrap();
}

fn main() {
    let args = Args::parse();
    run(args);
}
