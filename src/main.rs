mod config;
mod model;

use config::*;

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config(String::from("./config.cfg")) {
		Some(c) => c,
		None => panic!("internal error!"),
	};
	println!("\npath:");
	println!("{}", cfg.path());
	println!("\n\nignored:");
	for i in cfg.ignored() {
		print!("{} ", i);
	}
	println!("\n\nignored_suffix:");
	for i in cfg.ignored_suffix() {
		print!("{} ", i);
	}
}

