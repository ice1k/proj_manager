use std::io::{Read, Result, BufRead, BufReader};
use std::fs::{self, File, DirEntry};

fn visit_files(dir: &Path, func: Fn(&DirEntry)) -> Result<()> {
	if try!(fs::metadata(dir).is_dir()) {
		//
	}
	Ok(())
}
