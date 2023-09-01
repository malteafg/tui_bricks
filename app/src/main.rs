fn main() -> tui_bricks::error::Result<()> {
    env_logger::init();
    let mut stdout = std::io::stdout();
    tui_bricks::run(&mut stdout)
}
