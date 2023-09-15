#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! ```

use arboard::Clipboard;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	process::exit(0);
    }
    let text: &String = &args[1];
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
