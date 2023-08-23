use std::fmt::Display;
use std::io::Write;

use crate::input;
use crate::mode::Mode;
use crate::{data::Item, error::Result};

use crossterm::{
    cursor::{self, MoveToNextLine},
    queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

pub fn emit_line<W: Write, D: Display>(w: &mut W, line: D) -> Result<()> {
    queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn emit_dash<W: Write>(w: &mut W) -> Result<()> {
    emit_line(w, "---------------------------------------------")?;
    Ok(())
}

pub fn header<W: Write>(w: &mut W, header: &str) -> Result<()> {
    emit_dash(w)?;
    emit_iter(w, header.split("\n"))?;
    emit_dash(w)?;
    queue!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn default_header<W: Write>(w: &mut W) -> Result<()> {
    header(w, "Welcome to TUI Bricks")?;
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

    input::wait_for_u32(w)
}

pub fn input_string<W: Write>(w: &mut W, text: &str) -> Result<String> {
    emit_iter(w, text.split("\n"))?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    let result = input::wait_for_string(w)?.trim().to_string();

    queue!(w, cursor::Hide)?;
    Ok(result)
}

pub fn confirmation_prompt<W: Write>(w: &mut W, text: &str) -> Result<bool> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "(y)es or (n)o?")?;
    w.flush()?;

    loop {
        match input::wait_for_cmdchar()? {
            'y' => return Ok(true),
            'n' => return Ok(false),
            _ => {}
        }
    }
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
        let selected = input::wait_for_cmdchar()?;
        info!("{selected}");
        for (c, d) in options {
            if *c == selected {
                return Ok(d.clone());
            }
        }
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

pub fn construct_item_changes(old: &Item, new: &Item) -> String {
    let mut diff = String::new();
    if old.get_id() != new.get_id() {
        diff.push_str(&format!("Part ID: {} -> {}\n", old.get_id(), new.get_id()));
    }
    if old.get_alternative_ids() != new.get_alternative_ids() {
        diff.push_str(&format!(
            "Alternative IDs: {} -> {}\n",
            old.get_id(),
            new.get_id()
        ));
    }
    if old.get_name() != new.get_name() {
        diff.push_str(&format!(
            "Name: {:#?} -> {:#?}\n",
            old.get_name(),
            new.get_name()
        ));
    }
    if old.get_amount() != new.get_amount() {
        diff.push_str(&format!(
            "Amount: {:#?} -> {:#?}\n",
            old.get_amount(),
            new.get_amount()
        ));
    }
    if old.get_location() != new.get_location() {
        diff.push_str(&format!(
            "Locations: {:#?} -> {:#?}\n",
            old.get_location(),
            new.get_location()
        ));
    }
    diff
}

pub trait EmitMode {
    fn emit_mode<W: Write>(&self, w: &mut W) -> Result<()>;
}

impl EmitMode for Mode {
    fn emit_mode<W: Write>(&self, w: &mut W) -> Result<()> {
        clear(w)?;
        use Mode::*;
        match self {
            Default { info } => {
                default_header(w)?;
                queue!(w, Print(info), cursor::MoveToNextLine(2))?;
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
            }
        }
        Ok(())
    }
}
