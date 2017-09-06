use std::thread;

pub fn main() {
    let mut threads = vec![];

    for i in 0..10 {
        threads.push(thread::spawn(move || println!("Hello from thread {}", i)));
    }
    for t in threads {
        let _ = t.join();
    }
}
