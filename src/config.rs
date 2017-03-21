use model::*;

use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufRead, BufReader};
// use std::io::prelude::*;

fn open_file(path: StrType) -> Option<File> {
	match File::open(path) {
		Ok(f) => Some(f),
		_ => None,
	}
}

pub fn parse_config(path: StrType) -> Option<Config> {
	let file = match open_file(path.clone()) {
		Some(f) => f,
		None => return None,
	};
	let mut ignored = Vec::new();
	let mut ignored_suffix = Vec::new();
	let buf = BufReader::new(file);
	let mut proj_name = String::from("Unknown project name");
	let mut build = Vec::new();
	for ln in BufRead::lines(buf) {
		let mut ln = ln.unwrap_or(String::from(""));
		if ln.starts_with("ign:") {
			ignored.push(ln.drain(4..).collect());
		} else if ln.starts_with("ign-sfx:") {
			ignored_suffix.push(ln.drain(8..).collect());
		} else if ln.starts_with("build:") {
			build.push(ln.drain(6..).collect());
		} else if ln.starts_with("name:") {
			proj_name = ln.drain(5..).collect();
		} else {
			// ignore unknown commands
		}
	}
	Some(Config::new(
			proj_name,
			path,
			ignored,
			ignored_suffix,
			build
	))
}
