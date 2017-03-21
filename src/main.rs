mod config;
mod model;
mod files;

use std::path::Path;

use config::*;
use files::*;

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
		println!("{} ", i);
	}
	println!("\n\nignored_suffix:");
	for i in cfg.ignored_suffix() {
		println!("{} ", i);
	}
	visit_files(&cfg, Path::new("."), &visit_print);
}

