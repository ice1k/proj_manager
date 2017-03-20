use model::*;

use std::fs::File;
use std::path::{Path, Display};
use std::io::prelude::*;
use std::io::Error;

// to_string

pub fn reload() {
	//
}

fn load_file(path: str_ref) -> Option<File> {
	match File::open(path) {
		Ok(f) => Some(f),
		_ => None,
	}
}

pub fn parse_config(path: str_ref) -> Option<Config> {
	let path_obj = match Path::new(path).to_str() {
		Some(s) => s,
		None => return None,
	};
	let file = match load_file(path) {
		Some(f) => f,
		None => return None,
	};
	Some(Config::new("", path_obj))
}
