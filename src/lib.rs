use std::error::Error;
use std::fs;

pub struct Config {
	query: String,
	file_path: String,
}

impl Config {
	/// `build` is more semantic here since `new` is expected never to fail
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config { query, file_path })
	}
}

/// returns unit when no `Err` variants, and any kind of `Error` otherwise
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	println!("With text:\n{contents}");

	Ok(())
}
