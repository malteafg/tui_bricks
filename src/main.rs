fn main() -> tui_bricks::error::Result<()> {
    let path = tui_bricks::io::get_default_database_path()?;
    let test = tui_bricks::io::read_database_from_path(&path)?;

    let mut stdout = std::io::stdout();
    tui_bricks::run(&mut stdout, test)
}
