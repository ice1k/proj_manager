use model::*;

use std::fs::File;
use std::path::{Path};
use std::io::Read;
// use std::io::prelude::*;

// to_string

pub fn reload() {
	//
}

fn load_file(path: StrType) -> Option<File> {
	match File::open(path) {
		Ok(f) => Some(f),
		_ => None,
	}
}

pub fn parse_config(path: StrType) -> Option<Config> {
	let path_obj = match Path::new(path).to_str() {
		Some(s) => s,
		None => return None,
	};
	let mut file = match load_file(path) {
		Some(f) => f,
		None => return None,
	};
	let ignored_files = Vec::new();
	let mut content = Vec::new();
	file.read_to_end(&mut content);
	for line in content.split(|c| *c == '\n' as u8) {
		//
	}
	Some(Config::new("", path_obj, ignored_files))
}
