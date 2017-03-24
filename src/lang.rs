use std::path::Path;
// use std::ffi::OsStr;

trait Lang : ToString {
	fn match_str(&self, s: String) -> bool;

	/// match the path
	fn match_path<'a>(&self, path: &'a Path) -> bool {
		path
				.to_str()
				.map(|it| self.match_str(String::from(it)))
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
		s.ends_with(&self.suffix)
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
	($x: expr, $($y: expr), +) => {
		ComplexLang::new($x, vec!( $( String::from($y) , )+ ))
	}
}

// macro_rules! clang {
// 	($x: expr, $($y: expr), +) => {
// 		ComplexLang::new($x, vec!( $( $y , )+ ))
// 	}
// }

// o_O! old bad design
//
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
	let simple_langs = [
		slang!("Go", ".go"),
		slang!("Io", ".io"),
		slang!("Erlang", ".erl"),
		slang!("Lice", ".lice"),
		slang!("D", ".d"),
		slang!("Rust", ".rs"),
		slang!("Coq", ".v"),
		slang!("Clojure", ".clj"),
		slang!("Scala", ".scala"),
		slang!("C#", ".cs"),
		slang!("Ruby", ".rb"),
		slang!("Python", ".py"),
		slang!("MPS", ".mps"),
		slang!("Scheme", ".scm"),
		slang!("Racket", ".rkt"),
		slang!("Lisp", ".lisp"),
		slang!("XHTML", ".xhtml"),
		slang!("JavaScript", ".js"),
		slang!("TypeScript", ".ts"),
		slang!("CoffeeScript", ".coffee"),
		slang!("HobbyScript", ".hh"),
		slang!("Shell", ".sh"),
		slang!("CSS", ".css"),
		slang!("SASS", ".sass"),
		slang!("JSON", ".json"),
		slang!("Lua", ".lua"),
		slang!("PHP", ".php"),
		slang!("AIML", ".aiml"),
		slang!("TOML", ".toml"),
		slang!("INI", ".ini"),
		slang!("Swift", ".swift"),
		slang!("proj_manager config", "proj_config"),
		slang!("Git Ignore", ".gitignore"),
		slang!("Git Attributes", ".gitattributes"),
		slang!("PureScript", ".purs"),
	];
	let complex_langs = [
		clang!("Java", ".java", ".jar", ".war", ".aar", ".class"),
		clang!("C++", ".cpp", ".cc", ".hpp", ".C", ".cxx", ".hxx"),
		clang!("C", ".c", ".h"),
		clang!("XML", ".xml", ".iml", ".ipr", ".svg", ".icls"),
		clang!("HTML", "html", "htm"),
		clang!("Vim", ".vim", ".vimrc"),
		clang!("Markdown", ".md", ".markdown"),
		clang!("AsciiDoc", ".adoc", ".asciidoc"),
		clang!("Haskell", ".hs", ".lhs"),
		clang!("Kotlin", ".kt", ".kts"),
		clang!("Groovy", ".groovy", ".gradle"),
		clang!("EmacsLisp", ".el", ".emacs"),
		clang!("Batch", ".bat", ".cmd"),
		clang!("Visual Basic", ".vb", ".frm", ".vbs"),
		clang!("CMake", "CMakeLists.txt", ".cmake"),
		clang!("Manifest", ".MF", ".mf"),
	];
	for i in complex_langs.iter() {
		if i.match_path(p) {
			return i.to_string();
		}
	}
	for i in simple_langs.iter() {
		if i.match_path(p) {
			return i.to_string();
		}
	}
	String::from("Unknown")
}
