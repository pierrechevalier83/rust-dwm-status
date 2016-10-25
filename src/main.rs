use std::process::Command;
use std::time::Duration;
use std::thread;

extern crate chrono;
extern crate systemstat;

use systemstat::{Platform, System};

fn battery(sys: &System) -> String {
	if let Ok(bat) = sys.battery_life() {
        format!("bat {}%", bat.remaining_capacity * 100.)
	} else {
	    format!("bat _")
	}
}

fn cpu(sys: &System) -> String {
	if let Ok(load) = sys.load_average() {
	    format!("⚙ {}%", load.one)
	} else {
	    format!("⚙ _")
	}
}

fn date() -> String {
    // 2016-10-25 00:30
    chrono::Local::now().format("%F %R").to_string()
}

fn update_status(sys: &System) {
    let status = format!("{} | {} | {}", battery(sys), cpu(sys), date());
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
