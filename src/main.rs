mod config;
mod model;
mod files;
mod funcs;

use std::io::{Result, stdin};

use config::*;
use model::*;
use funcs::*;

fn repl(cfg: &Config) -> Result<()> {
	let name = cfg.proj_name();
	loop {
		let mut input = String::new();
		stdin()
				.read_line(&mut input)
				.expect("Failed to get input from stdin!");
		let i = input.trim();
		match i {
			"data" => print_meta(cfg),
			"ls" => print_files(cfg),
			"help" => print_help(),
			"exit" => go_die(),
			"line" => print_code_line_sum(cfg),
			// "cls" => clear_screen(),
			_ => println!("Unknown command: {}", i),
		}
	}
}

fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config(String::from("./proj_config")) {
		Some(c) => c,
		None => panic!("internal error!"),
	};
	println!("proj_manager v0.1.0, open source under GNU General Public License v3.0.");
	println!("See: https://github.com/ice1000/proj_manager");
	println!("Load success.");
	repl(&cfg);
}

