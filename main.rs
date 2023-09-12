#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! proconio = "0.4.5"
//! ```

use arboard::Clipboard;
// use proconio::input;

fn main() {
    let text: String = {
	let mut text = String::new();
	std::io::stdin().read_line(&mut text).unwrap();
	
	text.trim_end().to_owned()
    };
    
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
