use std::path::Path;
use std::ffi::OsStr;

trait Lang : ToString {
	fn match_str(&self, s: String) -> bool;

	/// match the path
	fn match_path<'a>(&self, path: &'a Path) -> bool {
		path
				.extension()
				.map(OsStr::to_str)
				.map(|it| self.match_str(String::from(it.unwrap_or(""))))
				.unwrap_or(false)
	}
}

struct SimpleLang {
	suffix: String,
	name: String,
}

impl Lang for SimpleLang {
	fn match_str(&self, s: String) -> bool {
		self.suffix.ends_with(&s)
	}
}

impl ToString for SimpleLang {
	fn to_string(&self) -> String {
		self.name.clone()
	}
}

impl SimpleLang {
	fn new<'a>(suffix: &'a str, name: &'a str) -> SimpleLang {
		SimpleLang {
			suffix: String::from(suffix),
			name: String::from(name),
		}
	}
}

// struct Cpp { }
// impl Lang for Cpp {
// 	fn match_str(s: String) -> bool {
// 		s.ends_with(".cpp") ||
// 				s.ends_with(".hpp") ||
// 				s.ends_with(".cc")
// 	}
// 	fn to_string() -> String { String::from("C++") }
// }

pub fn judge_lang_path(p: &Path) -> String {
	let s_langs = [
		SimpleLang::new(".java", "Java"),
		SimpleLang::new(".go", "Golang"),
		SimpleLang::new(".rs", "Rust"),
	];
	for i in &s_langs {
		if i.match_path(p) {
			return i.to_string();
		}
	}
	String::from("Unknown language")
}
