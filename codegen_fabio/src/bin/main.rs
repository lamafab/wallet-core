use std::env;
use system::{Result, start_parser};

fn main() -> Result<()> {
    if let Some(path) = env::args().nth(1) {
		let out = start_parser(&path)?;
		println!("{out}");
    } else {
		eprintln!("No path provided");
		std::process::exit(1);
	}

	Ok(())
}
