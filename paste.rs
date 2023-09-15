#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! ```

use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    print!("{}", clipboard.get_text().unwrap());
}
