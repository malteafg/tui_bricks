use std::collections::BTreeSet;
use std::fmt::Display;
use std::io::Write;

use crossterm::{
    cursor::{self, MoveToNextLine},
    queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

use crate::command::CmdChar;
use crate::error::{Error, Result};
use crate::input;

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

pub fn emit_iter<W: Write, D: Display>(
    w: &mut W,
    iter: impl Iterator<Item = D>,
) -> Result<()> {
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
    options: &BTreeSet<D>,
) -> Result<D> {
    let mut cmds: Vec<char> = Vec::new();
    ('a'..='z').for_each(|l| cmds.push(l));

    if options.len() > cmds.len() {
        panic!("Too many options to select from list");
    }

    let list: Vec<(char, D)> = options
        .iter()
        .enumerate()
        .map(|(i, o)| (cmds[i], o.clone()))
        .collect();

    emit_iter(w, text.split("\n"))?;
    emit_line(w, "Select from the list by typing the letter")?;
    queue!(w, MoveToNextLine(1))?;

    for (c, o) in list.iter() {
        emit_line(w, &format!("{}: {}", c, o))?;
    }

    w.flush()?;

    loop {
        let selected = input::wait_for_cmdchar()?;
        for (c, o) in list.iter() {
            if *c == selected {
                return Ok(o.clone());
            }
        }
    }
}

pub fn select_from_list_char<W: Write, D: Display + Clone + CmdChar>(
    w: &mut W,
    text: &str,
    options: &BTreeSet<D>,
) -> Result<D> {
    emit_iter(w, text.split("\n"))?;
    emit_line(w, "Select from the list by typing the letter")?;
    queue!(w, MoveToNextLine(1))?;
    for x in options.iter() {
        emit_line(w, &format!("{}: {}", x.get_char(), x.to_string()))?;
    }
    w.flush()?;

    loop {
        let selected = input::wait_for_cmdchar()?;
        for x in options.iter() {
            if x.get_char() == selected {
                return Ok(x.clone());
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

pub fn fzf_search(opts: &str) -> Result<String> {
    use std::io::Read;
    use std::process::{Command, Stdio};

    // Start the fzf process with stdin and stdout pipes
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Get a handle to the stdin and stdout of the fzf process
    let fzf_stdin = fzf.stdin.as_mut().ok_or(Error::ExternalCmdError)?;
    let fzf_stdout = fzf.stdout.as_mut().ok_or(Error::ExternalCmdError)?;

    // Write the input to the stdin of the fzf process
    fzf_stdin.write_all(opts.as_bytes())?;

    let mut res = String::new();
    fzf_stdout.read_to_string(&mut res)?;
    Ok(res.trim().to_string())
}
