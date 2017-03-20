mod config;
mod model;

use config::*;

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config("./Cargo.toml") {
		Some(c) => c,
		None => panic!("internal error!"),
	};
	println!("{}", cfg.get_path());
}

