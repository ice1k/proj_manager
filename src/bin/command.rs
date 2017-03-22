use std::process::Command;

/// 我不明白！
fn main() {
	match Command::new("call")
			// .env("PATH", ".")
			.arg("build")
			.output() {
		Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap()),
		_ => { },
	}
}