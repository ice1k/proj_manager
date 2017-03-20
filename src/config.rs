use model::Config;

use std::fs::File;
use std::path::{Path, Display};
use std::io::prelude::*;

pub fn parse_config(path: &str) -> Config {
	let f = Path::new(path);
	// let display = f.display();
	Config::new("", "")
}