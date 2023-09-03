use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	// the Ok variant matters, so we use unwrap_or_else to handle Err
	// variant or hold the value
	let config = Config::build(&args).unwrap_or_else(|error| {
		println!("Problem parsing arguments: {error}");
		// non-zero error code is conventional to signal an error
		process::exit(1);
	});

	// Ok variant doesn't matter, so use if let to throw away Ok variant and
	// handle Err variant
	if let Err(error) = minigrep::run(config) {
		println!("Application error: {error}");
		process::exit(1);
	}
}
