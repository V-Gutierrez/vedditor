use std::io::{stdin, Read};

fn main() {
    for byte in stdin().bytes() {
        match byte {
            Ok(b) => {
                print!("{}", b as char);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
