use std::thread;
use std::time::Duration;

pub fn main() {
    for _ in 0..10 {
        thread::spawn(|| println!("Hello from thread"));
    }
    thread::sleep(Duration::from_secs(1));
}
