use model::*;
use files::*;
use files::FileNode::*;
use lang::*;

use std::path::Path;
use std::fs::DirEntry;
use std::io::{Read, BufReader, BufRead};
use std::process::{exit, Command};

/// exit
pub fn go_die() -> ! {
	println!("Have a nice day :)");
	exit(0)
}

/// clear the screen
// pub fn clear_screen() {
// 	Command::new("cls").output().expect("Failed to run command: cls");
// }

/// print help
pub fn print_help() {
	println!("Commands:");
	println!("data           -- print the meta data stored in the cofiguration file.");
	println!("ls             -- print all the files.");
	println!("exit           -- exit project manager.");
	println!("help           -- print this doc.");
	println!("line           -- see how many lines of code is here in your project.");
	println!("git            -- print git status.");
}

/// print meta data
/// stored in the cofiguration file
pub fn print_meta(cfg: &Config) {
	println!("Name:");
	println!("\t{}", cfg.proj_name());
	println!("Path:");
	println!("\t{}", cfg.path());
	println!("Ignored:");
	for i in cfg.ignored() {
		println!("\t{} ", i);
	}
	println!("Ignored Suffix:");
	for i in cfg.ignored_suffix() {
		println!("\t{} ", i);
	}
	println!("Build Script:");
	for i in cfg.build() {
		println!("\t{} ", i);
	}
}

/// list the files
#[allow(unused_must_use)]
pub fn print_files(cfg: &Config) {
	visit_files(&cfg, Path::new("."), &|e: &DirEntry| {
		let p = e.path();
		println!(
			"{:<indent_1$} => Language: {}",
			p.display(),
			judge_lang_path(&p),
			indent_1 = cfg.indent_ls_1() as usize
		);
	});
}

/// print how many lines of code is here
pub fn print_code_line_new(cfg: &Config) {
	let root = build_file_tree(cfg, Path::new("."));
	fn rec_visit(cfg: &Config, node: FileNode, sum: u64) -> u64 {
		match node {
			FileLeaf(fw) => {
				let br = BufReader::new(fw.f());
				let lines = br.lines().count() as u64;
				let br = BufReader::new(fw.f());
				println!(
					"In {:<indent_1$} => {:<indent_2$} lines, {:<indent_3$} per line.",
					fw.name(),
					lines,
					br.bytes().count() as u64 / lines,
					indent_1 = cfg.indent_line_1() as usize,
					indent_2 = cfg.indent_line_2() as usize,
					indent_3 = cfg.indent_line_3() as usize
				);
				sum + lines
			},
			Directory(vec) => {
				let mut sum = sum;
				//				let vec: Vec<FileNode> = *vec;
				for i in vec {
					sum += rec_visit(cfg, i, 0)
				}
				sum
			}
		}
	}
	println!("Total: {} lines of code.", rec_visit(cfg, root, 0));
}

pub fn print_git_data(cfg: &Config) {
	// println!("In project {}:", cfg.proj_name());
	let status = match Command::new("git")
			.arg("status")
			.output() {
		Ok(o) => o.stdout,
		_ => {
			println!("Cannot run \'git status\' in {}.", cfg.proj_name());
			return;
		}
	};
	let branches = match Command::new("git")
			.arg("branch")
			.output() {
		Ok(o) => o.stdout,
		_ => {
			println!("Cannot run \'git branch\' in {}.", cfg.proj_name());
			return;
		}
	};
	let branches = String::from_utf8(branches.clone())
			.unwrap_or(String::from("no git branches found."));
	println!("Branches:");
	for ln in branches.lines() {
		println!("\t{}", ln);
	}
	let status = String::from_utf8(status.clone())
			.unwrap_or(String::from("no git status found."));
	let mut first = true;
	for ln in status.lines() {
		// let ln = ln.trim();
		if first {
			if ln.starts_with("fatal:") {
				break;
			} else {
				println!("Git root detected in {}.", cfg.proj_name());
			}
		}
		first = false;
		if !ln.starts_with("  (use \"git ") &&
				!ln.starts_with("no changes added to commit") &&
				!ln.trim().is_empty() {
			println!("{}", ln);
		}
	}
	if !first {
		println!("Running git gc..");
		match Command::new("git")
				.arg("gc")
				.output() {
			Ok(_) => println!("Git gc finished."),
			_ => {
				println!("Cannot run \'git gc\' in {}.", cfg.proj_name());
				return;
			}
		}
	} else {
		println!("No git root detected in {}.", cfg.proj_name());
	}
}

// #[allow(dead_code)]
pub fn build_proj(cfg: &Config) {
	for i in cfg.build() {
		println!("Running: {}", i);
	}
}

