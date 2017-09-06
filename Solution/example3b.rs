use std::thread;
use std::time::Duration;

pub fn main() {
    for i in 0..10 {
        thread::spawn(move || println!("Hello from thread {}", i));
    }
    thread::sleep(Duration::from_secs(1));
}
