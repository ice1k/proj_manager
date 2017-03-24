use std::io::{Result};
use std::fs::{self, DirEntry, File};
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

pub enum FileNode {
	Directory(Vec<FileNode>),
	FileLeaf(File)
}

pub fn build_file_tree(cfg: &Config, dir: &Path) -> FileNode {
	if fs::metadata(dir).unwrap().is_dir() {
		let mut v = vec![];
		for e in fs::read_dir(dir).unwrap() {
			let e = e.unwrap();
			let e_path = e.path();
			let s = String::from(e
					.file_name()
					.to_str()
					.expect("WTF!"));
			if !cfg.is_ignored(&s) && !cfg.is_ignored_suffix(&s) {
				v.push(build_file_tree(cfg, &e_path));
			}
		}
		FileNode::Directory(v)
	} else {
		FileNode::FileLeaf(File::create(dir
				.to_str()
				.unwrap())
				.unwrap()
		)
	}
}
