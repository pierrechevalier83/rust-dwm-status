use std::process::Command;

fn main() {
    Command::new("xsetroot")
			.arg("-name")
			.arg("Hello, World!")
			.spawn()
			.expect("Failed to run command");
}
