use std::io::{Result};
use std::fs::{self, DirEntry};
use std::path::{Path};

use model::*;

// pub fn reload<F>(path: &str) -> F
// 		where F: Fn(Fn(&DirEntry)) -> () {
// 	|func: Fn(&DirEntry)| {
// 		visit_files(Path::new(path), func).unwrap()
// 	}
// }

pub fn visit_files(cfg: &Config, dir: &Path, func: &Fn(&DirEntry)) -> Result<()> {
	if fs::metadata(dir)?.is_dir() {
		for e in fs::read_dir(dir)? {
			let e = e?;
			let e_path = e.path();
			let s = String::from(e.file_name().to_str().expect("WTF!"));
			if !cfg.is_ignored(&s) && !cfg.is_ignored_suffix(&s) {
				if fs::metadata(e_path.clone())?.is_dir() {
					visit_files(cfg, &e_path, func)?;
				} else {
					func(&e);
				}
			}
		}
	} else {
		let e = fs::read_dir(dir)?.next().unwrap()?;
		func(&e);
	}
	Ok(())
}
