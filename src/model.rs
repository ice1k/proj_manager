type str_ref = &'static str;

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
}
