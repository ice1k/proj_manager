pub type str_ref = &'static str;

// #[derive()]
pub struct Config {
	proj_name: str_ref,
	path: str_ref,
	//
}

impl Config {
	pub fn new(
				proj_name: str_ref,
				path: str_ref) -> Config {
		Config {
			proj_name: proj_name,
			path: path
		}
	}

	pub fn get_path(&self) -> str_ref {
		self.path
	}
}
