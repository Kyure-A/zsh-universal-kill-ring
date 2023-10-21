//! # main

mod history;
mod kill_ring;
use crate::kill_ring::kill_ring::*;
use std::process;

fn main () -> () {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
	process::exit(1);
    }
    
    let command: &String = &args[1];

    if command == "copy" && args.len() == 3 {
	let text: &String = &args[2];
	kill(text);
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
