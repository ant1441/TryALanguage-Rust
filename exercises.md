# Exercise 1 - Hello World!

Write a basic hello world in Rust!

## Note

* `rustc file.rs` produces an executable called `file`.
* Normally, for larger projects you would use `cargo`. Cargo also does package management.
* Try `cargo new hello_world` and `cargo run` in the directory created.

# Exercise 2 - Ownership

Create a function which takes a String (call `.to_string()` on a string literal), and prints it out.

Make sure it compiles and runs as you expect.

Call that function a second time in the main function, and see why it fails to compile.

Understand how to fix that, and what it means.

# Exercise 3 - Threading

Create a program which uses the `std::thread::spawn` function to print from multiple threads in parallel.
Modify your program to print the number of the thread.

## Note
May need `move` on closure to capture index.
Also `thread::sleep`.
Also `JoinHandle::join`.

# Exercise 4 - Shared mutability

Create a mutable integer which can be shared across threads, and add up 1 to 10 with 10 threads.

Try it with Atomics, or a Mutex.

## Note
* `std::sync::atomic::{AtomicUsize, Ordering}`;
* `AtomicUsize` has the `fetch_add` method.
* Atomics - Use `Ordering::SeqCst`.
* `Mutex` - `Deref` for `MutexGaurd`.

# Exercise 5 - Error Handling

* Create a small program that prints if a given file exists or not.
* Use suitable error handling.
* Extend your program to read and print the contents of the file.

## Note

* `::std::fs::File::open` is a function which returns a `Result<File, Error>`

# Exercise 6 - Networking

Create a basic TCP server which responds with "hello world"

## Note

* `std::net::TcpListener`

# Exercise 7

Create a TCP server which responds with a hash of the received data.
Extend this by handling each request in it's own thread.

# Exercise other

* Error handling
* Generics
* Strings
