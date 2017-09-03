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
* `|var| { func(var) }` is a closure

# Types

* `u8`, `i8` - unsigned and signed 8 bit integers (also 16, 32, 64 and size)
* `f32`, `f64` - single / double precision floating point number
* `bool` - `true` or `false`
* `char` - a *Unicode Scalar Value*
* `(u8, u8)`, `[u8]` - Tuples and arrays
* Structs and ADTs - `Option<T>`, `Result<T, E>`

# Ownership & Borrowing

* `let a = 1`, the current scope then 'owns' `a`
* `func(a)` passes ownership of `a` to the scope of `func`
* `func(&a)`, in this case the scope of `func` is 'borrowing' `a`, the current scope gets it back after the function completes

# Mutability

* `let a = 1; a += 1;` is invalid, as `a` is mutable by default
* `let mut a = 1; a += 1;` specifies `a` is mutable
* `func(&mut a)` passes a mutable reference to `func`

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
