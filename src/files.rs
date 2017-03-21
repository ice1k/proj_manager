use std::io::{Read, Result, BufRead, BufReader, Error};
use std::fs::{self, File, DirEntry};
use std::path::{Path};

use model::*;

// pub fn reload<F>(path: &str) -> F
// 		where F: Fn(Fn(&DirEntry)) -> () {
// 	|func: Fn(&DirEntry)| {
// 		visit_files(Path::new(path), func).unwrap()
// 	}
// }

pub fn visit_files(cfg: &Config, dir: &Path, func: &Fn(&DirEntry)) -> Result<()> {
	if try!(fs::metadata(dir)).is_dir() {
		for e in try!(fs::read_dir(dir)) {
			let e = try!(e);
			let e_path = e.path();
			let s = String::from(e.file_name().to_str().expect("WTF!"));
			if !cfg.is_ignored(&s) && !cfg.is_ignored_suffix(&s) {
				if try!(fs::metadata(e_path.clone())).is_dir() {
					try!(visit_files(cfg, &e_path, func));
				} else {
					func(&e);
				}
			}
		}
	} else {
		let e = try!(try!(fs::read_dir(dir)).next().unwrap());
		func(&e);
	}
	Ok(())
}
