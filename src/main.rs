mod config;
mod model;

use config::*;

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config(String::from("./config.cfg")) {
		Some(c) => c,
		None => panic!("internal error!"),
	};
	println!("{}", cfg.path());
	for i in cfg.ignored() {
		print!("{} ", i);
	}
	println!("");
	for i in cfg.ignored_suffix() {
		print!("{} ", i);
	}
}

