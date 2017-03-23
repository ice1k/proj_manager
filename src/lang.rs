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

macro_rules! to_str_by_name_impl {
	($x: ident) => {
		impl ToString for $x {
			fn to_string(&self) -> String {
				self.name.clone()
			}
		}
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

impl SimpleLang {
	fn new<'a>(name: &'a str, suffix: &'a str) -> SimpleLang {
		SimpleLang {
			suffix: String::from(suffix),
			name: String::from(name),
		}
	}
}

struct ComplexLang {
	suffixes: Vec<String>,
	name: String,
}

impl Lang for ComplexLang {
	fn match_str(&self, s: String) -> bool {
		for i in &self.suffixes {
			if s.ends_with(i) {
				return true;
			}
		}
		false
	}
}

impl ComplexLang {
	fn new<'a>(name: &'a str, suffixes: Vec<String>) -> ComplexLang {
		ComplexLang {
			name: String::from(name),
			suffixes: suffixes,
		}
	}
}

to_str_by_name_impl!(SimpleLang);
to_str_by_name_impl!(ComplexLang);

macro_rules! slang {
	($x: expr, $y: expr) => {
		SimpleLang::new($x, $y)
	}
}

macro_rules! clang {
	($x: expr, $($y: expr),+) => {
		ComplexLang::new($x, vec!( $(y)+ ))
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
	let langs = [
		slang!("Java", ".java"),
		slang!("Golang", ".go"),
		slang!("Rust", ".rs"),
		slang!("Coq", ".v"),
		slang!("Clojure", ".clj"),
		slang!("Scala", ".scala"),
		slang!("C#", ".cs"),
		slang!("Ruby", ".rb"),
		clang!("C++", "cpp", "hpp"),
	];
	for i in &langs {
		if i.match_path(p) {
			return i.to_string();
		}
	}
	String::from("Unknown language")
}
