//! # judge
//! This module provides functions to determine whether command line arguments are correct.

pub mod judge {
    pub fn is_copy(command: &String, args: &Vec<String>) -> bool {
	command == "copy" && args.len() == 3
    }

    pub fn is_yank(command: &String, args: &Vec<String>) -> bool {
	command == "yank" && args.len() == 2
    }

    pub fn is_yank_pop(command: &String, args: &Vec<String>) -> bool {
	command == "yank-pop" && args.len() == 2
    }
}
