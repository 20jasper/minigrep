use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	let Config { query, file_path } = Config::new(&args);

	let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

	println!("With text:\n{contents}");
}

struct Config {
	query: String,
	file_path: String,
}

impl Config {
	fn new(args: &[String]) -> Config {
		let query = args[1].clone();
		let file_path = args[2].clone();

		Config { query, file_path }
	}
}
