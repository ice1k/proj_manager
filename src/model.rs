pub type StrType = &'static str;

// #[derive()]
pub struct Config {
	proj_name: StrType,
	path: StrType,
	ignored: Vec<StrType>,
	ignored_suffix: Vec<StrType>,
}

impl Config {
	pub fn new(
				proj_name: StrType,
				path: StrType,
				ignored: Vec<StrType>,
				ignored_suffix: Vec<StrType>) -> Config {
		Config {
			proj_name: proj_name,
			path: path,
			ignored: ignored,
			ignored_suffix: ignored_suffix,
		}
	}

	pub fn path(&self) -> StrType {
		self.path
	}

	pub fn proj_name(&self) -> StrType {
		self.proj_name
	}
}
