#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! ```

use arboard::Clipboard;
use std::env;
use std::process;

fn copy(text: &String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}

fn paste() {
    let mut clipboard = Clipboard::new().unwrap();
    print!("{}", clipboard.get_text().unwrap());
}

fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
	process::exit(1);
    }
    
    let command: &String = &args[1];

    if command == "copy" {
	let text: &String = &args[2];
	copy(text);
	process::exit(0);
    }

    else if command == "paste" {
	paste();
	process::exit(0);
    }
}
