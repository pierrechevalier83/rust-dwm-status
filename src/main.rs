use std::process::Command;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;

extern crate chrono;
extern crate num_cpus;

fn cpu() -> String {
    let mut s = String::new();
    File::open("/proc/loadavg").unwrap().read_to_string(&mut s).unwrap();
    s.truncate(4); // We only want the current cpu time
    let load: f32 = s.parse().unwrap();
    format!("⚙ {}%", (100. * load) as usize / num_cpus::get())
}

fn date() -> String {
    // 2016-10-25 00:30
    chrono::Local::now().format("%F %R").to_string()
}

fn update_status() {
    let status = format!("{} | {}", cpu(), date());
    Command::new("xsetroot")
        .arg("-name")
        .arg(status)
        .spawn()
        .expect("Failed to run command");
}

fn main() {
    loop {
        update_status();
        thread::sleep(Duration::new(1, 0)); // second
    }
}
