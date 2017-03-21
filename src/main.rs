mod config;
mod model;
mod files;
mod funcs;

use std::io::{Result, stdin};

use config::*;
use model::*;
use funcs::*;
use std::process::exit;

fn repl(cfg: &Config) -> Result<()> {
	let name = cfg.proj_name();
	loop {
		let mut input = String::new();
		stdin().read_line(&mut input);
		let i = input.trim();
		match i {
			"data" => print_meta(cfg),
			"ls" => print_files(cfg),
			"help" => print_help(),
			"exit" => exit(0),
			_ => println!("Unknown command: {}", i),
		}
	}
}

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config(String::from("./config.cfg")) {
		Some(c) => c,
		None => panic!("internal error!"),
	};
	println!("Load success.");
	repl(&cfg);
}

