pub type StrType = String;
// pub type StrType = &'static str;

// #[derive()]
pub struct Config {
	proj_name: StrType,
	path: StrType,
	ignored: Vec<StrType>,
	ignored_suffix: Vec<StrType>,
	build: StrType,
}

impl Config {
	pub fn new(
				proj_name: StrType,
				path: StrType,
				ignored: Vec<StrType>,
				ignored_suffix: Vec<StrType>,
				build: StrType) -> Config {
		Config {
			proj_name: proj_name,
			path: path,
			ignored: ignored,
			ignored_suffix: ignored_suffix,
			build: build,
		}
	}

	pub fn path(&self) -> StrType {
		self.path.clone()
	}

	pub fn proj_name(&self) -> StrType {
		self.proj_name.clone()
	}
}
