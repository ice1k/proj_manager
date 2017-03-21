pub type StrType = String;
// pub type StrType = &'static str;

// #[derive()]
pub struct Config {
	proj_name: StrType,
	path: StrType,
	ignored: Vec<StrType>,
	ignored_suffix: Vec<StrType>,
	build: Vec<StrType>,
}

impl Config {
	pub fn new(
				proj_name: StrType,
				path: StrType,
				ignored: Vec<StrType>,
				ignored_suffix: Vec<StrType>,
				build: Vec<StrType>) -> Config {
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

	pub fn ignored(&self) -> Vec<StrType> {
		self.ignored.clone()
	}

	pub fn ignored_suffix(&self) -> Vec<StrType> {
		self.ignored_suffix.clone()
	}

	pub fn build(&self) -> Vec<StrType> {
		self.build.clone()
	}

	pub fn is_ignored(&self, name: &StrType) -> bool {
		self.ignored.contains(name)
	}

	pub fn is_ignored_suffix(&self, name: &StrType) -> bool {
		for sfx in &self.ignored_suffix {
			if name.ends_with(sfx) {
				return true;
			}
		}
		false
	}
}
