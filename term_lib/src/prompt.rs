use std::collections::BTreeSet;
use std::fmt::Display;
use std::io::Write;

use crossterm::{cursor, queue};

use crate::command::Command;
use crate::display::*;
use crate::input;
use crate::{Error, Result};

pub fn input_u32<W: Write>(w: &mut W, text: &str) -> Result<u32> {
    iter(w, text.split("\n"))?;
    line(w, "(Input should be a number)")?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    input::wait_for_u32(w)
}

pub fn input_string<W: Write>(w: &mut W, text: &str) -> Result<String> {
    iter(w, text.split("\n"))?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    let result = input::wait_for_string(w)?.trim().to_string();

    queue!(w, cursor::Hide)?;
    Ok(result)
}

pub fn confirmation<W: Write>(w: &mut W, text: &str) -> Result<bool> {
    iter(w, text.split("\n"))?;
    line(w, "(y)es or (n)o?")?;
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

    iter(w, text.split("\n"))?;
    line(w, "Select from the list by typing the letter")?;
    newline(w, 1)?;

    for (c, o) in list.iter() {
        line(w, &format!("{}: {}", c, o))?;
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

pub fn select_cmd<W: Write, D: Clone + Command>(
    w: &mut W,
    text: &str,
    options: &BTreeSet<D>,
) -> Result<D> {
    iter(w, text.split("\n"))?;
    line(w, "Select from the list by typing the letter")?;
    newline(w, 1)?;

    for x in options.iter() {
        line(w, x.display_as_cmd())?;
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
