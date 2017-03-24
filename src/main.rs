mod config;
mod model;
mod files;
mod funcs;
mod lang;

use std::io::{Result, stdin};

use config::*;
use model::*;
use funcs::*;

#[allow(unused_must_use, unused_variables)]
fn repl<'a>(cfg: &'a Config) -> Result<()> {
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
			"line" => print_code_line_new(cfg),
//			"line" => print_code_line_sum(cfg),
			"git" => print_git_data(cfg),
			"build" => build_proj(cfg),
			// "cls" => clear_screen(),
			_ => println!("Unknown command: {}", i),
		}
	}
}

#[allow(unused_must_use)]
fn main() {
	// println!("{}", parse_config("./Cargo.toml"));
	let cfg = match parse_config(String::from("./proj_config")) {
		Some(c) => c,
		None => {
			println!("Load failed DA★ZE...\nplease create a file naming 'proj_config' here.");
			return;
			// panic!("internal error!");
		},
	};
	// println!("✔proj_manager v0.1.0, open source under GNU General Public License v3.0.");
	// println!("😘See: https://github.com/ice1000/proj_manager");
	// println!("😜Load success DA★ZE.");
	// println!("👀Input \"help\" to get help.");
	println!("proj_manager v0.1.0, open source under GNU General Public License v3.0.");
	println!("See: https://github.com/ice1000/proj_manager");
	println!("Load success DA★ZE!.");
	println!("Input \"help\" to get help.");
	println!("");

	repl(&cfg);
}

