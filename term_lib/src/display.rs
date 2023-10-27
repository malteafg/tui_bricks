use std::fmt::Display;
use std::io::Write;

use crossterm::{
    cursor, queue,
    style::{Print, ResetColor},
    terminal,
};

use crate::command::Command;
use crate::Result;

pub fn line<W: Write, D: Display>(w: &mut W, line: D) -> Result<()> {
    queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn dash<W: Write>(w: &mut W) -> Result<()> {
    line(w, "---------------------------------------------")?;
    Ok(())
}

pub fn header<W: Write>(w: &mut W, header: &str) -> Result<()> {
    dash(w)?;
    iter(w, header.split("\n"))?;
    dash(w)?;
    queue!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn iter<W: Write, D: Display>(
    w: &mut W,
    iter: impl Iterator<Item = D>,
) -> Result<()> {
    for line in iter {
        queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    }
    Ok(())
}

pub fn possible_cmds<'a, W: Write, D: Command + 'a>(
    w: &mut W,
    iter: impl Iterator<Item = &'a D>,
) -> Result<()> {
    for line in iter {
        queue!(w, Print(line.display_as_cmd()), cursor::MoveToNextLine(1))?;
    }
    Ok(())
}

pub fn newline<W: Write>(w: &mut W, n: u16) -> Result<()> {
    queue!(w, cursor::MoveToNextLine(n))?;
    Ok(())
}

pub fn clear<W: Write>(w: &mut W) -> Result<()> {
    queue!(
        w,
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}
