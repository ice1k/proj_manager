pub type StrType = &'static str;

// #[derive()]
pub struct Config {
	proj_name: StrType,
	path: StrType,
	ignored: Vec<StrType>,
}

impl Config {
	pub fn new(
				proj_name: StrType,
				path: StrType,
				ignored: Vec<StrType>) -> Config {
		Config {
			proj_name: proj_name,
			path: path,
			ignored: ignored,
		}
	}

	pub fn get_path(&self) -> StrType {
		self.path
	}

	pub fn get_proj_name(&self) -> StrType {
		self.proj_name
	}
}
