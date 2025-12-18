use rebrickable_client::ClientDB;
use rebrickable_database_api::{Part, PartId, RebrickableDB};
use rebrickable_server_api::query::{ColorGetType, GetItem, PartGetType};
use utils::{DisplayShortExt, PathExt};

use clap::Parser;

use std::fs;
use std::path::{Path, PathBuf};

static NO_IMAGE: &[u8] = include_bytes!("../assets/no_image.png");

// TODO: use the database so we can also copy images based on name of the part.
// TODO have argument to specify path to images, such that we can enter a tui logo.
#[derive(Parser, Debug)]
pub struct Args {
    /// The item to update
    #[command(subcommand)]
    item: GetItem,

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

fn try_copy_part_image(part: &Part, base_path: impl AsRef<Path>, dst_path: impl AsRef<Path>) {
    if try_copy_image(&base_path, &part.part_record.part_num, &dst_path) {
        return;
    }

    for parent_part in part.parent_rels.keys() {
        if try_copy_image(&base_path, &parent_part, &dst_path) {
            return;
        }
    }

    fs::write(&dst_path, NO_IMAGE).unwrap();
}

fn handle_with_db<D: RebrickableDB>(
    database: &D,
    item: GetItem,
    base_path: impl AsRef<Path>,
    dst_path: impl AsRef<Path>,
) {
    match item {
        GetItem::Part {
            part: PartGetType::Id { id },
        } => {
            let part = database.part_from_id(&id).unwrap();
            try_copy_part_image(&part, base_path, dst_path);
            println!("{}", part.short());
        }
        GetItem::Part {
            part: PartGetType::Name { name },
        } => {
            let part = database.part_from_name(&name).unwrap();
            try_copy_part_image(&part, base_path, dst_path);
            println!("{}", part.short());
        }
        GetItem::Color {
            color: ColorGetType::Id { id },
        } => {
            let color = database.color_from_id(&id).unwrap();
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("{}", color.short());
        }
        GetItem::Color {
            color: ColorGetType::Name { name },
        } => {
            let color = database.color_from_name(&name).unwrap();
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("{}", color.short());
        }
        GetItem::Element { id } => {
            let element = database.element_from_id(&id).unwrap();
            let part = database
                .part_from_id(&element.element_record.part_num)
                .unwrap();
            try_copy_part_image(&part, base_path, dst_path);
            println!("{}", element.short());
            println!("{}", part.short());
        }
    }
}

fn handle(item: GetItem, base_path: impl AsRef<Path>, dst_path: impl AsRef<Path>) {
    match item {
        GetItem::Part {
            part: PartGetType::Id { id },
        } => {
            if !try_copy_image(&base_path, &id, &dst_path) {
                fs::write(&dst_path, NO_IMAGE).unwrap();
            }
            println!("Part id: {}", id);
        }
        GetItem::Part {
            part: PartGetType::Name { name },
        } => {
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("Part name: {}", name);
        }
        GetItem::Color {
            color: ColorGetType::Id { id },
        } => {
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("Color id: {}", id);
        }
        GetItem::Color {
            color: ColorGetType::Name { name },
        } => {
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("Color name: {}", name);
        }
        GetItem::Element { id } => {
            fs::write(&dst_path, NO_IMAGE).unwrap();
            println!("Element id: {}", id);
        }
    }
}

fn run(mut args: Args) {
    let base_path = match args.images_path.take() {
        Some(base_path) => base_path,
        None => {
            let mut base_path = PathBuf::data_dir();
            base_path.push("part_images");
            base_path
        }
    };
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

    match ClientDB::new() {
        Ok(database) => {
            handle_with_db(&database, args.item, base_path, dst_path);
        }
        Err(_) => {
            handle(args.item, base_path, dst_path);
        }
    };
}

fn main() {
    let args = Args::parse();
    run(args);
}
