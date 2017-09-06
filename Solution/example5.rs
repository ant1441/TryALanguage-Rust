use std::fs::File;

pub fn main() {
    if File::open("example5.rs").is_ok() {
        println!("File exists!");
    } else {
        println!("File does not exist!");
    }
}
