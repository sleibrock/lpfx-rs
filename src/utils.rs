
use std::time::Duration;
use std::thread;

pub fn sleep_seconds(n: u64) {
    thread::sleep(Duration::from_secs(n));
}

pub fn sleep_millis(n: u64) {
    thread::sleep(Duration::from_millis(n));
}
