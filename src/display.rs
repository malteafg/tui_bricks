use std::fmt::Display;
use std::io::Write;

use crate::{error::Result, io, mode::Mode};

use crossterm::{
    cursor::{self, MoveToNextLine, MoveToPreviousLine},
    style::{Print, ResetColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

macro_rules! queue_test {
    ($writer:expr $(, $command:expr)* $(,)?) => {{

        // This allows the macro to take both mut impl Write and &mut impl Write.
        Ok($writer)
            $(.and_then(|writer| QueueableCommand::queue(writer, $command)))*
            .map(|_| ())
    }}
}

macro_rules! execute_test {
    ($writer:expr $(, $command:expr)* $(,)? ) => {{
        // Queue each command, then flush
        queue_test!($writer $(, $command)*)
            .and_then(|()| {
                ::std::io::Write::flush($writer)
            })
    }}
}

pub fn emit_line<D: Display>(w: &mut dyn Write, line: D) -> Result<()> {
    queue_test!(w, Print(line), cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn emit_dash(w: &mut dyn Write) -> Result<()> {
    emit_line(w, "---------------------------------------------")?;
    Ok(())
}

pub fn header(w: &mut dyn Write, header: &str) -> Result<()> {
    emit_dash(w)?;
    emit_iter(w, header.split("\n"))?;
    emit_dash(w)?;
    queue_test!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn default_header(w: &mut dyn Write) -> Result<()> {
    header(w, "Welcome to TUI Bricks")?;
    Ok(())
}

pub fn emit_iter<D: Display>(w: &mut dyn Write, iter: impl Iterator<Item = D>) -> Result<()> {
    for line in iter {
        queue_test!(&mut *w, Print(line), cursor::MoveToNextLine(1))?;
    }
    Ok(())
}

pub fn input_u32(w: &mut dyn Write, text: &str) -> Result<u32> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "(Input should be a number)")?;
    queue_test!(&mut *w, cursor::Show)?;
    w.flush()?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if let Ok(u32_input) = input.trim().parse() {
            queue_test!(w, cursor::Hide)?;
            return Ok(u32_input);
        } else {
            execute_test!(
                &mut *w,
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            )?;
        }
    }
}

pub fn input_string(w: &mut dyn Write, text: &str) -> Result<String> {
    emit_iter(w, text.split("\n"))?;
    queue_test!(&mut *w, cursor::Show)?;
    w.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let result = input.trim().to_string();

    queue_test!(w, cursor::Hide)?;
    Ok(result)
}

pub fn confirmation_prompt(w: &mut dyn Write, text: &str) -> Result<bool> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "(y)es or (n)o?")?;
    w.flush()?;

    loop {
        match io::wait_for_char()? {
            'y' => return Ok(true),
            'n' => return Ok(false),
            _ => {}
        }
    }
}

pub fn select_from_list<D: Display + Clone>(
    w: &mut dyn Write,
    text: &str,
    options: &[(char, D)],
) -> Result<D> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "Select from the list by typing the letter")?;
    queue_test!(&mut *w, MoveToNextLine(1))?;
    for (c, d) in options {
        emit_line(w, &format!("{}: {}", c, d.to_string()))?;
    }
    w.flush()?;

    loop {
        let selected = io::wait_for_char()?;
        for (c, d) in options {
            if *c == selected {
                return Ok(d.clone());
            }
        }
        // execute!(w, MoveToPreviousLine(1), Clear(ClearType::CurrentLine))?;
    }
}

pub fn clear(w: &mut dyn Write) -> Result<()> {
    w.queue(ResetColor)?;
    // queue!(
    //     w,
    //     ResetColor,
    //     terminal::Clear(ClearType::All),
    //     cursor::Hide,
    //     cursor::MoveTo(0, 0)
    // )?;

    Ok(())
}

pub trait EmitMode {
    fn emit_mode(&self, w: &mut dyn Write) -> Result<()>;
}

impl EmitMode for Mode {
    fn emit_mode(&self, w: &mut dyn Write) -> Result<()> {
        clear(w)?;
        use Mode::*;
        match self {
            Default { info } => {
                default_header(w)?;
                queue_test!(w, Print(info), cursor::MoveToNextLine(2))?;
            }
            DisplayItem { item } => {
                header(w, &format!("Viewing item with part ID {}", item.get_id()))?;
                emit_iter(w, item.to_string().split("\n"))?;
            }
            EditItem { item } => {
                header(
                    w,
                    &format!("Now editing item with part ID {}", item.get_id()),
                )?;
                emit_iter(w, item.to_string().split("\n"))?;
                emit_line(w, "use any of the following commands to edit the item")?;
            }
        }
        Ok(())
    }
}
