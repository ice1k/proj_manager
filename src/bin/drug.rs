use std::process::Command;

#[allow(dead_code)]
fn this_function_is_like_drug() {
	let status = match Command::new("git")
			.arg("status")
			.output() {
		Ok(o) => o.stdout,
		_ => panic!("WTF???"),
	};
	println!("fuck:");
	println!("{}", match String::from_utf8(status) {
		Ok(o) => o,
		_ => panic!("Invalid utf8 String DA★ZE!"),
	});
}

fn main() {
	//
}