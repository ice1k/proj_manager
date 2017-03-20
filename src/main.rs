mod config;
mod model;

use config::*;

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	parse_config("./Cargo.toml");
}

