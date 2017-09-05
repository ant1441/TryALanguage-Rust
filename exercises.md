# Exercise 1 - Hello World!

Write a basic hello world in Rust!

## Note

* `rustc file.rs` produces an executable called `file`.
* Normally, for larger projects you would use `cargo`. Cargo also does package management.
* Try `cargo new --bin hello_world` and `cargo run` in the directory created.

## Example

``` rust
fn main() {
    println!("Example!");
}
```

# Exercise 2 - Ownership

Create a function which takes a String (call `.to_string()` on a string literal), and prints it out.

Make sure it compiles and runs as you expect.

Call that function a second time in the main function, and see why it fails to compile.

Understand how to fix that, and what it means.

## Note

Trying this with numbers may give a different result.
They implement the `Copy` trait, which means they can be automatically copied to pass into a function.

## Example

``` rust
fn main() {
    let value = 123;
    foo(value);
}

fn foo(val: u32) {
    println!("Val: {}", val);
}
```

# Exercise 3 - Threading

Create a program which uses the `std::thread::spawn` function to print from multiple threads in parallel.
Modify your program to print the number of the thread.

## Note
* May need `move` on closure to capture index.
* Also you may wish to use `thread::sleep` or `JoinHandle::join`.

## Example

``` rust
use std::thread;
use std::time::Duration;
fn main() {
    thread::spawn(|| println!("A thread!"));
    thread::sleep(Duration::from_secs(1));
}
```

# Exercise 4 - Shared mutability

Create a mutable integer which can be shared across threads, and add up 1 to 10 with 10 threads.

Try it with Atomics, or a Mutex.

The Rust compiler guarantees that your code will not have any data races, so you can parallelise your code with confidence.
(Though it wont catch _general_ race conditions, such as deadlocks)

## Note
* `std::sync::atomic::{AtomicUsize, Ordering}`;
* `AtomicUsize` has the `fetch_add` method.
* Atomics - Use `Ordering::SeqCst`.
* `Mutex` - `Deref` for `MutexGaurd`.

## Example

``` rust
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let xs = vec![];
    let axs = Arc::new(Mutex::new(xs));

    for _ in 0..10 {
        let axs = axs.clone();
        thread::spawn(move || {
            let mut v = axs.lock().unwrap();
            v.push(1)
        });
    }

    thread::sleep(Duration::from_secs(1));
    println!("Sum: {:?}", axs);
}
```

# Exercise 5 - Error Handling

* Create a small program that prints if a given file exists or not.
* Use suitable error handling.
* Extend your program to read and print the contents of the file.

## Note

* `::std::fs::File::open` is a function which returns a `Result<File, Error>`

## Example

``` rust
use std::net::TcpStream;
use std::io::Read;
fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
        let mut data = String::new();
        if stream.read_to_string(&mut data).is_err() {
            println!("Error reading from server...");
        };
        println!("{}", data);
    } else {
        println!("Error connecting to server...");
    }
}
```

# Exercise 6 - Networking

Create a basic TCP server which responds with "hello world"

## Note

* `std::net::TcpListener`

# Exercise 7

* Create a TCP server which responds with a hash of the received data.
* Extend this by handling each request in it's own thread.
* Extend this further by storing computed requests in a central cache (Try `std::collections::HashMap` in a `Mutex`)

## Example

You may use this hash function, or (if you have an internet connection, try using the [md5](https://crates.io/crates/md5) Crate.

``` rust
use std::hash::Hash;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```
