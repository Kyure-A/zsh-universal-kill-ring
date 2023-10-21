//! kill_ring

pub mod kill_ring {
    use crate::history::history::*;
    use arboard::Clipboard;

    fn copy(text: &String) -> () {
	let mut clipboard = Clipboard::new().unwrap();
	clipboard.set_text(text).unwrap();
    }

    pub fn kill(text: &String) -> () {
	copy(text);
	make_history(text).unwrap();
    }

    pub fn yank() -> () {
	let mut clipboard = Clipboard::new().unwrap();
	print!("{}", clipboard.get_text().unwrap());
    }

    pub fn yank_pop() -> () {
	let clipboard = show_history();
	print!("{}", clipboard);
    }
}
