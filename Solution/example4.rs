use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub fn main() {

    let mut ts = vec![];

    let sum = Arc::new(AtomicUsize::new(0));

    for i in 0..10 {
        let sum = sum.clone();
        ts.push(thread::spawn(move || {
            sum.fetch_add(i, Ordering::SeqCst);
        }));
    }

    for t in ts {
        let _ = t.join();
    }
    println!("Sum: {}", sum.load(Ordering::Relaxed));
}
