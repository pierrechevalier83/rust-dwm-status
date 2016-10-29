use std::process::Command;
use std::time::Duration;
use std::thread;

extern crate chrono;
extern crate systemstat;

use systemstat::{Platform, System};

fn plugged(sys: &System) -> String {
    if let Ok(plugged) = sys.on_ac_power() {
        if plugged {
		    format!("{}","🔌 ✓")
		} else {
		    format!("{}","🔌 ✘")
		}
	} else {
		format!("{}","🔌_")
	}
}

fn battery(sys: &System) -> String {
	if let Ok(bat) = sys.battery_life() {
        format!("🔋 {:.1}%", bat.remaining_capacity * 100.)
	} else {
	    format!("🔋 _")
	}
}

fn ram(sys: &System) -> String {
    if let Ok(mem) = sys.memory() {
	    let pmem = mem.platform_memory;
		let used = pmem.total - pmem.free - pmem.buffer - pmem.shared;
		format!("▯ {}", used)
	} else {
	    format!("▯ _")
    }
}

fn cpu(sys: &System) -> String {
	if let Ok(load) = sys.load_average() {
	    format!("⚙ {:.2}%", load.one)
	} else {
	    format!("⚙ _")
	}
}

fn date() -> String {
    // 2016-10-25 00:30
    chrono::Local::now().format("📆 %a, %d %h ⸱ 🕓 %R").to_string()
}

fn update_status(sys: &System) {
    let status = format!("{} ⸱ {} ⸱ {} ⸱ {} ⸱ {}", plugged(sys), battery(sys), ram(sys), cpu(sys), date());
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
