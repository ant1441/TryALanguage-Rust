use std::fs::File;
use std::io::Read;

pub fn main() {
    if let Ok(mut f) = File::open("example5.rs") {
        let mut contents = String::new();
        let res = f.read_to_string(&mut contents);
        if let Err(e) = res {
            println!("Error reading file: {}", e);
        } else {
            println!("{}", contents);
        }
    } else {
        println!("File does not exist!");
    }
}
