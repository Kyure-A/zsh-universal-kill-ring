#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! dialoguer = { version = "0.10.4", features = ["fuzzy-select"] }
//! ```

use arboard::Clipboard;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::{process, io::Write};
use std::io::prelude::*;

const HISTORY_PATH: &str = "./kill-ring";

fn make_history(text: &String) -> std::io::Result<()> {
    let output: std::fs::File = std::fs::OpenOptions::new()
	.write(true)
        .create(true)
        .open(HISTORY_PATH)
        .unwrap();
    let mut writer = std::io::BufWriter::new(output);
    writer.write_all(text.as_bytes())?;
    writer.write_all(b"\n")?;

    Ok(())
}

fn get_history() -> std::io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let file = std::fs::File::open(HISTORY_PATH)?;
    let histories = std::io::BufReader::new(file);

    for history in histories.lines() {
	let history = history?;
	result.push(history);
    }

    Ok(result)
}

fn copy(text: &String) -> () {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}

fn yank() -> () {
    let mut clipboard = Clipboard::new().unwrap();
    print!("{}", clipboard.get_text().unwrap());
}

fn yank_pop() -> () {
    let histories: Vec<String> = get_history().unwrap();
    let selector = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("kill-ring")
	.items(&histories)
        .default(0)
        .interact()
        .unwrap();

    print!("{}", histories[selector]);
}

fn main () -> () {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
	process::exit(1);
    }
    
    let command: &String = &args[1];

    if command == "copy" && args.len() == 3 {
	let text: &String = &args[2];
	copy(text);
	make_history(text).unwrap();
	process::exit(0);
    }

    else if command == "yank" && args.len() == 2 {
	yank();
	process::exit(0);
    }

    else if command == "yank-pop" && args.len() == 2 {
	yank_pop();
	process::exit(0);
    }
}
