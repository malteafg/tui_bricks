use clap::Parser;

use std::path::PathBuf;

static NO_IMAGE: &[u8] = include_bytes!("../assets/no_image.png");

// TODO: use the database so we can also copy images based on name of the part.
#[derive(Parser, Debug)]
pub struct Args {
    /// The id of the part whose image is to be updated.
    part_id: String,

    /// The path to copy the file to. This defaults to the cache directory of the os with a folder
    /// called tui_bricks.
    #[arg(long)]
    dst_path: Option<PathBuf>,

    /// 
    #[arg(long)]
    images_path: Option<PathBuf>,
}

fn run(args: Args) {
    if args.dst_path.is_none() {

    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", utils::workspace_root().display());
    println!("{}", std::env::var("CARGO_WORKSPACE_DIR").unwrap());
}
