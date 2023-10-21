//! # main

mod kill_ring;
mod history;
mod judge;
use judge::judge::{is_yank, is_yank_pop};

use crate::kill_ring::kill_ring::*;
use crate::judge::judge::*;
use std::process::exit;

fn main () -> () {
    let args: Vec<String> = std::env::args().collect();
    
    let command: &String = &args[1];

    if is_copy(command, &args) {
	let text: &String = &args[2];
	kill(text);
    }

    if is_yank(command, &args) {
	yank();
	exit(0);
    }

    if is_yank_pop(command, &args) {
	yank_pop();
	exit(0);
    }

    exit(1);
}
