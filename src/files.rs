use std::io::{Read, Result, BufRead, BufReader};
use std::fs::{self, File, DirEntry};

pub fn visit_files(dir: &Path, func: Fn(&DirEntry)) -> Result<()> {
	if try!(fs::metadata(dir).is_dir()) {
		for e in try!(fs::read_dir(dir)) {
			let e = try!(e);
			if try!(fs::metadata(e.path())).is_dir() {
				try!(visit_files(&e.path(), func));
			} else {
				func(e);
			}
		}
	} else {
		let e = try!(try!(fs::read_dir(dir)).next().unwrap());
		func(e);
	}
	Ok(())
}
