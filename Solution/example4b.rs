use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn main() {

    let mut ts = vec![];

    let sum = Arc::new(Mutex::new(0));

    for i in 0..10 {
        let sum = sum.clone();
        ts.push(thread::spawn(move || {
            let mut sum = sum.lock().unwrap();
            *sum += i;
        }));
    }

    for t in ts {
        let _ = t.join();
    }
    println!("Sum: {}", *sum.lock().unwrap());
}
