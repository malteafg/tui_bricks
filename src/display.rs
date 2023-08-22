use std::fmt::Display;
use std::io::Write;

use crate::{error::Result, io, state::Mode};

use crossterm::{
    cursor::{self, MoveToNextLine, MoveToPreviousLine},
    execute, queue,
    style::{Print, ResetColor},
    terminal::{self, Clear, ClearType},
};

pub fn emit_line<W: Write, D: Display>(w: &mut W, line: D) -> Result<()> {
    queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn header<W: Write>(w: &mut W) -> Result<()> {
    emit_line(w, "--------------------")?;
    emit_line(w, " TUI Bricks")?;
    emit_line(w, "--------------------")?;
    queue!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn emit_iter<W: Write, D: Display>(w: &mut W, iter: impl Iterator<Item = D>) -> Result<()> {
    for line in iter {
        queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    }
    Ok(())
}

pub fn input_u32<W: Write>(w: &mut W, text: &str) -> Result<u32> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "(Input should be a number)")?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if let Ok(u32_input) = input.trim().parse() {
            queue!(w, cursor::Hide)?;
            return Ok(u32_input);
        } else {
            execute!(w, MoveToPreviousLine(1), Clear(ClearType::CurrentLine))?;
        }
    }
}

pub fn input_string<W: Write>(w: &mut W, text: &str) -> Result<String> {
    emit_iter(w, text.split("\n"))?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let result = input.trim().to_string();

    queue!(w, cursor::Hide)?;
    Ok(result)
}

pub fn select_from_list<W: Write, D: Display + Clone>(
    w: &mut W,
    text: &str,
    options: &[(char, D)],
) -> Result<D> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "Select from the list by typing the letter")?;
    queue!(w, MoveToNextLine(1))?;
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

pub fn clear<W: Write>(w: &mut W) -> Result<()> {
    queue!(
        w,
        ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}

pub trait EmitMode {
    fn emit_mode<W: Write>(&self, w: &mut W) -> Result<()>;
}

impl EmitMode for Mode {
    fn emit_mode<W: Write>(&self, w: &mut W) -> Result<()> {
        use Mode::*;
        match self {
            Default { info } => queue!(w, Print(info), cursor::MoveToNextLine(1))?,
            DisplayItem { item } => emit_iter(w, item.to_string().split("\n"))?,
        }
        Ok(())
    }
}
