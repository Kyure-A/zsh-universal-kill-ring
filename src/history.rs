//! ## history

pub mod history {
    use std::fs::File;
    use std::io::Write;
    use std::io::prelude::*;
    use std::path::PathBuf;
    use home_dir::*;
    use dialoguer::{theme::ColorfulTheme, FuzzySelect};
    
    fn get_history_path() -> PathBuf {
	let history_str: String = std::env::var("UNIKRHIST").unwrap_or("~/unikrhist".to_string());
	let history_path: PathBuf = std::path::Path::new(&history_str).expand_home().unwrap();

	return history_path;
    }
    
    pub fn make_history(text: &String) -> std::io::Result<()> {
	let output: File = std::fs::OpenOptions::new()
	    .append(true)
            .create(true)
            .open(get_history_path())
	    .unwrap();
	
	let mut writer = std::io::BufWriter::new(output);
	writer.write_all(text.as_bytes())?;
	writer.write_all(b"\n")?;

	Ok(())
    }

    fn get_history() -> std::io::Result<Vec<String>> {
	let mut result: Vec<String> = Vec::new();
	let file: File = std::fs::File::open(get_history_path())?;
	let histories = std::io::BufReader::new(file);

	for history in histories.lines() {
	    let history: String = history?;
	    result.push(history);
	}

	Ok(result)
    }

    pub fn show_history() -> String {
	let histories: &Vec<String> = &get_history().unwrap();
	let selector = FuzzySelect::with_theme(&ColorfulTheme::default())
	    .with_prompt("kill-ring")
	    .items(&histories)
	    .default(0)
	    .interact()
	    .unwrap();
	
	return histories[selector].clone();
    }
}
