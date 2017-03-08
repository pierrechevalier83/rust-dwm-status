use std::process::Command;
use std::time::Duration;
use std::thread;

#[macro_use]
extern crate chan;
extern crate chan_signal;

extern crate chrono;
extern crate systemstat;

use chan_signal::Signal;
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
	    format!("⚙ {:.2}", load.one)
	} else {
	    format!("⚙ _")
	}
}

fn date() -> String {
    chrono::Local::now().format("📆 %a, %d %h ⸱ 🕓 %R").to_string()
}

fn status(sys: &System) -> String {
    format!("{} ⸱ {} ⸱ {} ⸱ {} ⸱ {}", plugged(sys), battery(sys), ram(sys), cpu(sys), date())
}

fn update_status(status: &String) {
    let _ = Command::new("xsetroot")
        .arg("-name")
        .arg(status)
		.spawn(); // Don't panic if we fail! We'll do better next time!
}

fn run(_sdone: chan::Sender<()>) {
    let sys = System::new();
    loop {
        update_status(&status(&sys));
        thread::sleep(Duration::new(10, 0)); // seconds
    }
}

fn main() {
    // Signal gets a value when the OS sent a INT or TERM signal.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    // When our work is complete, send a sentinel value on `sdone`.
    let (sdone, rdone) = chan::sync(0);
    // Run work.
    ::std::thread::spawn(move || run(sdone));

    // Wait for a signal or for work to be done.
    chan_select! {
        signal.recv() -> signal => {
            update_status(&format!("rust-dwm-status stopped with signal {:?}.", signal));
        },
        rdone.recv() => {
            update_status(&"rust-dwm-status: done.".to_string());
        }
    }
}

