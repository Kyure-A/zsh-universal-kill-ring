#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! arboard = "3.2.1"
//! proconio = "0.4.5"
//! ```

use arboard::Clipboard;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: &String = &args[1]; // ここ 0-indexed じゃないの罠すぎるだろ
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
