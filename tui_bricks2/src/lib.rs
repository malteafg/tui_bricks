use std::io::Write;

pub struct State<W: Write> {
    db: usize,
    mode: Box<dyn Mode<W>>,
}

impl<W: Write> State<W> {
    // Get possible cmds
    // Wait for a cmd to be entered
    // Call handle_cmd on the set mode
    //   Can hang and render if it needs input from user
    // Render the set mode
}

pub trait Mode<W: Write> {
    // fn handle_cmd(&mut self) -> Box<dyn Mode<W>>;
    fn render(&self, w: &mut W);
}

pub trait Cmd {
    fn exec(&self);
}
