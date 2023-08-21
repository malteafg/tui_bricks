use std::fmt::Display;
use std::io::Write;

use crate::{error::Result, state::Mode};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

pub fn emit_line<W: Write, D: Display>(w: &mut W, line: D) -> Result<()> {
    queue!(w, Print(line))?;
    Ok(())
}

pub fn header<W: Write>(w: &mut W) -> Result<()> {
    emit_line(w, "--------------------")?;
    emit_line(w, " TUI Bricks")?;
    emit_line(w, "--------------------")?;
    Ok(())
}

pub fn emit_iter<W: Write, D: Display>(w: &mut W, iter: impl Iterator<Item = D>) -> Result<()> {
    for line in iter {
        queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    }
    Ok(())
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
