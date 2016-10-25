use std::process::Command;
use std::time::Duration;
use std::thread;

extern crate chrono;
extern crate systemstat;

use systemstat::{Platform, System};

fn cpu(sys: &System) -> String {
	let load = sys.load_average().unwrap().one;
	format!("⚙ {}%", load)
}

fn date() -> String {
    // 2016-10-25 00:30
    chrono::Local::now().format("%F %R").to_string()
}

fn update_status(sys: &System) {
    let status = format!("{} | {}", cpu(sys), date());
    Command::new("xsetroot")
        .arg("-name")
        .arg(status)
        .spawn()
        .expect("Failed to run command");
}

fn main() {
    let sys = System::new();
    loop {
        update_status(&sys);
        thread::sleep(Duration::new(1, 0)); // second
    }
}
