fn func(s: &String) {
    println!("Received: '{}'", s);
}

pub fn main() {
    let s = "hello world".to_string();
    func(&s);
    func(&s);
}
