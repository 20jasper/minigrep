use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	// we care about the Ok variant, so we use unwrap_or_else to handle Err
	// variant or hold the value
	let config = Config::build(&args).unwrap_or_else(|error| {
		println!("Problem parsing arguments: {error}");
		// non-zero error code is conventional to signal an error
		process::exit(1);
	});

	// we don't care about Ok variant, so use if let to throw away Ok variant and
	// handle Err variant
	if let Err(error) = run(config) {
		println!("Application error: {error}");
		process::exit(1);
	}
}

// returns unit when ok, and any error otherwise
fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	println!("With text:\n{contents}");

	Ok(())
}

struct Config {
	query: String,
	file_path: String,
}

impl Config {
	// `build` is more semantic here since `new` is expected never to fail
	fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config { query, file_path })
	}
}
