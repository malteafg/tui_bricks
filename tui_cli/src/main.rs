use database::{Database, DatabaseI, PartNum};
use utils;

use std::error::Error;
use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parts_path = utils::data_dir();
    parts_path.push("parts.csv");
    dbg!(&parts_path.display());

    let mut colors_path = utils::data_dir();
    colors_path.push("colors.csv");

    let mut elements_path = utils::data_dir();
    elements_path.push("elements.csv");

    let database = Database::new(&parts_path, &colors_path, &elements_path);

    let dst_path = utils::cache_dir().join("displayed_image.png");
    let images_path = utils::data_dir().join("part_images");
    let update_image_cmd = format!(
        "tui_bricks_update_image {{}} --dst-path=\"{}\" --images-path=\"{}\"",
        dst_path.display(),
        images_path.display()
    );

    let mut child = Command::new("fzf")
        // .arg("--bind=focus:execute(sh -c '[ -f ../raw_data/parts_red/{}.png ] && cp ../raw_data/parts_red/{}.png ../raw_data/test_image.png' sh {})")
        // .arg(&format!(
        //     "--bind=focus:execute({} &>/dev/null &)",
        //     update_image_cmd
        // ))
        // .arg("--preview=(echo {})")
        .arg(&format!("--preview=({} && echo {{}})", update_image_cmd))
        .arg("--preview-window=up:30%:wrap")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;

        for key in database.iter_part_num() {
            // Attempt to write key
            match writeln!(stdin, "{}", key) {
                Ok(_) => {
                    stdin.flush()?; // Optional but helps
                }
                Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                    // fzf exited early — stop writing
                    eprintln!("fzf exited early (Broken pipe), stopping write loop");
                    break;
                }
                Err(e) => return Err(Box::new(e)), // Other unexpected error
            }
        }
    }

    // Step 4: Read selected key from fzf stdout
    let output = child.wait_with_output()?;
    // dbg!(String::from_utf8_lossy(&output.stdout));
    let selected_key: PartNum = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
        .into();

    // Step 5: Use selected key
    if let Some(value) = database.part_from_num(&selected_key) {
        println!("You selected: {} => {:?}", selected_key, value);
    } else {
        println!("Key not found: {}", selected_key);
    }

    Ok(())
}
