use model::*;

use std::fs::File;
// use std::path::{PathBuf};
use std::io::{BufRead, BufReader};
// use std::io::prelude::*;

/// returns an opened file
fn open_file<'a>(path: &'a String) -> Option<File> {
	match File::open(path.clone()) {
		Ok(f) => Some(f),
		_ => None,
	}
}

/// parse the configuration file
pub fn parse_config(path: String) -> Option<Config> {
	let file = match open_file(&path) {
		Some(f) => f,
		None => return None,
	};
	let mut ignored = Vec::new();
	let mut ignored_suffix = Vec::new();
	let buf = BufReader::new(file);
	let mut proj_name = String::from("Unknown project name");
	let mut build = Vec::new();
	let mut indent_line_1 = 32;
	let mut indent_line_2 = 4;
	let mut indent_line_3 = 4;
	let mut indent_ls_1 = 32;
	// let signs = [
	// 	"ign:",
	// 	"ign-sfx:",
	// 	"build:",
	// 	"name:",
	// 	"idt-line-1:",
	// 	"idt-line-2:"
	// ];
	for ln in BufRead::lines(buf) {
		let mut ln = ln.unwrap_or(String::from(""));
		// if ln.starts_with("#") {
		// 	continue;
		// }
		if ln.starts_with("ign:") {
			ignored.push(ln.drain(4..).collect());
		} else if ln.starts_with("ign-sfx:") {
			ignored_suffix.push(ln.drain(8..).collect());
		} else if ln.starts_with("build:") {
			build.push(ln.drain(6..).collect());
		} else if ln.starts_with("name:") {
			proj_name = ln.drain(5..).collect();
		} else if ln.starts_with("idt-line-1:") {
			let s: String = ln.drain(11..).collect();
			indent_line_1 = s.parse().unwrap();
		} else if ln.starts_with("idt-line-2:") {
			let s: String = ln.drain(11..).collect();
			indent_line_2 = s.parse().unwrap();
		} else if ln.starts_with("idt-line-3:") {
			let s: String = ln.drain(11..).collect();
			indent_line_3 = s.parse().unwrap();
		} else if ln.starts_with("idt-ls-1:") {
			let s: String = ln.drain(9..).collect();
			indent_ls_1 = s.parse().unwrap();
		} else {
			// ignore unknown commands
		}
	}
	Some(Config::new(
		proj_name,
		path,
		ignored,
		ignored_suffix,
		build,
		indent_line_1,
		indent_line_2,
		indent_line_3,
		indent_ls_1,
	))
}
