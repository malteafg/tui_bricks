use utils;

use clap::Parser;

use std::fs;
use std::path::PathBuf;

static NO_IMAGE: &[u8] = include_bytes!("../assets/no_image.png");

// TODO: use the database so we can also copy images based on name of the part.
// TODO have argument to specify path to images, such that we can enter a tui logo.
#[derive(Parser, Debug)]
pub struct Args {
    /// The id of the part whose image is to be updated.
    part_id: String,

    /// The path to copy the file to. This defaults to the cache directory of the os with a folder
    /// called tui_bricks.
    #[arg(long)]
    dst_path: Option<PathBuf>,

    /// The path where images of parts are located.
    #[arg(long)]
    images_path: Option<PathBuf>,
}

fn run(mut args: Args) {
    let mut dst_path = match args.dst_path.take() {
        Some(dst_path) => dst_path,
        None => utils::cache_dir(),
    };
    if dst_path.extension().is_none() {
        dst_path.push("displayed_image.png");
    }
    if let Some(parent) = dst_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let mut image_path = match args.images_path.take() {
        Some(image_path) => image_path,
        None => {
            let mut image_path = utils::data_dir();
            image_path.push("part_images");
            image_path
        }
    };

    image_path.push(args.part_id);
    image_path.set_extension("png");

    if fs::exists(&image_path).unwrap() {
        fs::copy(&image_path, &dst_path).unwrap();
    } else {
        fs::write(&dst_path, NO_IMAGE).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    run(args);
}
