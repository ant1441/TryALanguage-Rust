# Basic syntax

``` rust
pub fn main() {
    println!("Hello World!");
}
```

* `main` is a function
* `println!` is a macro
* `pub` declares its scope
* `main` is called by default in an executable
* `|var| { func(var) }` is a closure, and _borrows_ its encolsed variables (eg. `var`)
* `move |var| { func(var) }` is a closure which takes ownership of its enclosed variables
* Loops: Rust has the usual `for` and `while` loops. But also you can just `loop` for an infinite loop:

``` rust
for i in 0..10 {
   foo(i)
}
while do_thing {
   foo(i)
}
loop {
   foo(i)
}
```

# Types

* `u8`, `i8` - unsigned and signed 8 bit integers (also 16, 32, 64 and size)
* `f32`, `f64` - single / double precision floating point number
* `bool` - `true` or `false`
* `char` - a *Unicode Scalar Value*
* `str` - A _string slice_, usually seen as &str, a reference to some UTF-8 data stored somewhere else. eg. &'static is stored in the binary output of the program
* `String` - 'Owned' string, which is often heap allocated
* `(u8, u8)`, `[u8]` - Tuples and arrays
* Structs and ADTs - `Option<T>`, `Result<T, E>`

# Ownership & Borrowing

* `let a = 1`, the current scope then 'owns' `a`
* `func(a)` passes ownership of `a` to the scope of `func`
* `func(&a)`, in this case the scope of `func` is 'borrowing' `a`, the current scope gets it back after the function completes
* The type of ownership a funciton wants should be specified in the type signature too: `fn foo(val: Option<bool>)` versus `fn foo(val: &Option<bool>)`
* `func(&mut a)` passes a 'mutable reference'. This allows modification of the value `a` by the function. (Don't forget the type signature: `fn foo(val: &mut Option<bool>)`)

# Mutability

* `let a = 1; a += 1;` is invalid, as `a` is immutable by default
* `let mut a = 1; a += 1;` specifies `a` is mutable
* `func(&mut a)` passes a mutable reference to `func`

# References and Smart Pointers

* `T` (eg. `let val = 1`) is an 'Owned' value on the stack (possibly _containing_ a heap object, eg. `Vec`)
* `&T` (eg `&val`) is a 'reference' to the value. Similar (but *not* the same) as a pointer in other languages
* `Box<T>`, `Rc<T>`, are examples of _Smart Pointers_. `Box<T>` can be used for allocating on the Heap. `RC<T>` implementes reference counting

# Lifetimes

``` rust
fn foo(a: &T) {
    thread::spawn(move || loop { println!("a: {}", a) });
}
```

This function will not work, as it is borrowing `a` for the 'lifetime' of the function scope.
However, this function scope creates a thread that may live forever, so the function scope may not "let go" of `a`.
This is a problem, because `a` could be some heap allocated memory which immediately gets freed.

To work around this, you can specify a lifetime requirement to the parameter with the `'lifetime` syntax.
Ie. `foo(a: &'l T)`.
In this case the `'static` lifetime can be used, which requires that any `a` passed to this function lives as long as the execution of the program itself.

# Error Handling

* Many functions will return `Result<T, E>`, which has variants of `Ok(T)` and `Err(E)`
* The result can be extracted with `.unwrap()` or `if let Ok(val)`

# Traits

Traits are how Rust allows generic code.
Any struct can implement a trait, and a function can take a parameter of type 'Trait'.

Eg. The `ToString` trait defines the `to_string()` method, which must return a String.

# Further Reading

For further reading, read [The Rust Programming Language](https://doc.rust-lang.org/book/), and for advanced reading [The Rustonomicon](https://doc.rust-lang.org/nomicon/).
