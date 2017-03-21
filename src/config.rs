use model::*;

use std::fs::File;
use std::path::{Path};
use std::io::{BufRead, BufReader};
// use std::io::prelude::*;

// to_string

pub fn reload() {
	//
}

fn open_file(path: StrType) -> Option<File> {
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
	let file = match open_file(path) {
		Some(f) => f,
		None => return None,
	};
	let ignored = Vec::new();
	let ignored_suffix = Vec::new();
	let buf = BufReader::new(file);
	for ln in BufRead::lines(buf) {
		let ln = ln.unwrap_or(String::from(""));
		if ln.starts_with("ignore:") {
			// ignore.push(ln.sub)
		} else if ln.starts_with("ignore-suffix:") {
			//
		} else if ln.starts_with("") {
			//
		}
	}
	Some(Config::new("", path_obj, ignored, ignored_suffix))
}
