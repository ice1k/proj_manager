use model::*;
use files::*;

use std::path::Path;
use std::fs::DirEntry;

fn visit_print(e: &DirEntry) {
	println!("{}", e.path().display());
}

fn visit_code_line_sum(e: &DirEntry) {
	println!("{}", e.path().display());
}

pub fn print_help() {
	println!("commands:");
	println!("data           -- prints the meta data stored in the cofiguration file.");
	println!("ls             -- prints the files.");
	println!("exit           -- exit project manager.");
	println!("help           -- print this doc.");
}

pub fn print_meta(cfg: &Config) {
	println!("name:");
	println!("\t{}", cfg.proj_name());
	println!("path:");
	println!("\t{}", cfg.path());
	println!("ignored:");
	for i in cfg.ignored() {
		println!("\t{} ", i);
	}
	println!("ignored_suffix:");
	for i in cfg.ignored_suffix() {
		println!("\t*.{} ", i);
	}
}

pub fn print_files(cfg: &Config) {
	visit_files(&cfg, Path::new("."), &visit_print);
}

pub fn print_code_line_sum(cfg: &Config) {
	visit_files(&cfg, Path::new("."), &visit_code_line_sum);
}
