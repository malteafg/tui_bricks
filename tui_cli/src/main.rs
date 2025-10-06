use database::{Database, DatabaseI, PartNum};

use std::error::Error;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    parts_path.push("../raw_data/parts.csv");

    let mut colors_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    colors_path.push("../raw_data/colors.csv");

    let mut elements_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    elements_path.push("../raw_data/elements.csv");

    let database = Database::new(&parts_path, &colors_path, &elements_path);

    println!("datbase loaded");

    // Step 2: Spawn fzf
    // Just watch and read the test file to observe a change to fzf focus
    let mut child = Command::new("fzf")
        .arg("--bind=focus:execute-silent(echo {} > test_file.txt)")
        // .arg("--bind=focus:execute(echo {})")
        // .arg("cp ../raw_data/parts_red/{1}.png ../raw_data/test_image.png") // sxiv will display the image from the selected key
        // .arg("--preview-window=up:30%:wrap") // Optional: Makes the preview window appear above the fzf window
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
    dbg!(&output.stdout);
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
