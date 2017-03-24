// pub type String = String;
// pub type String = &'static str;

use files::FileNode;

#[allow(dead_code)]
pub struct Config {
	proj_name: String,
	path: String,
	ignored: Vec<String>,
	ignored_suffix: Vec<String>,
	build: Vec<String>,
	indent_line_1: u8,
	indent_line_2: u8,
	indent_line_3: u8,
	indent_ls_1: u8,
}

#[allow(dead_code)]
impl Config {
	pub fn new(
		proj_name: String,
		path: String,
		ignored: Vec<String>,
		ignored_suffix: Vec<String>,
		build: Vec<String>,
		indent_line_1: u8,
		indent_line_2: u8,
		indent_line_3: u8,
		indent_ls_1: u8,
	) -> Config {
		Config {
			proj_name: proj_name,
			path: path,
			ignored: ignored,
			ignored_suffix: ignored_suffix,
			build: build,
			indent_line_1: indent_line_1,
			indent_line_2: indent_line_2,
			indent_line_3: indent_line_3,
			indent_ls_1: indent_ls_1,
		}
	}

	pub fn path(&self) -> String {
		self.path.clone()
	}

	pub fn proj_name(&self) -> String {
		self.proj_name.clone()
	}

	pub fn ignored(&self) -> Vec<String> {
		self.ignored.clone()
	}

	pub fn ignored_suffix(&self) -> Vec<String> {
		self.ignored_suffix.clone()
	}

	pub fn build(&self) -> Vec<String> {
		self.build.clone()
	}

	pub fn indent_line_1(&self) -> u8 {
		self.indent_line_1
	}

	pub fn indent_line_2(&self) -> u8 {
		self.indent_line_2
	}

	pub fn indent_line_3(&self) -> u8 {
		self.indent_line_3
	}

	pub fn indent_ls_1(&self) -> u8 {
		self.indent_ls_1
	}

	pub fn is_ignored(&self, name: &String) -> bool {
		self.ignored.contains(name)
	}

	pub fn is_ignored_suffix(&self, name: &String) -> bool {
		for sfx in &self.ignored_suffix {
			if name.ends_with(sfx) {
				return true;
			}
		}
		false
	}
}
