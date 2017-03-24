use std::process::Command;

fn this_function_is_like_drug() {
	let status = match Command::new("git")
			.arg("status")
			.output() {
		Ok(o) => {
			//
		},
		_ => {
			//
		}
	};
	println!("fuck");
}

fn main() {
	//
}