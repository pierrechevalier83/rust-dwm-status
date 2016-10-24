use std::process::Command;

extern crate chrono;

fn date() -> String {
    // 2016-10-25 00:30
	chrono::Local::now().format("%F %R").to_string()
}

fn main() {
    Command::new("xsetroot")
			.arg("-name")
			.arg(date())
			.spawn()
			.expect("Failed to run command");
}
