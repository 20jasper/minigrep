use std::env;
use std::fs;
use std::process;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		// non-zero error code is conventional to signal an error
		process::exit(1);
	});

	run(config);
}

fn run(config: Config) {
	let contents =
		fs::read_to_string(config.file_path).expect("Should have been able to read the file");

	println!("With text:\n{contents}");
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
