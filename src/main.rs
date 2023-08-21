fn main() -> tui_bricks::error::Result<()> {
    let mut stdout = std::io::stdout();
    tui_bricks::run(&mut stdout)
}
