fn foo(a: &T) {
    thread::spawn(move || loop {
                      println!("a: {}", a)
                  });
}

fn main() {
    let s = vec![1, 2, 3, 4, 5];
    foo(&s);

    free(s);
    loop {}
}
