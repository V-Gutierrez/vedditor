use std::io::{stdin, stdout, Read};

use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    
    // bitwise AND operation
    byte & 0b0001_1111
}

fn finish_with_error(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in stdin().bytes() {
        match byte {
            Ok(byte) => {
                let char = byte as char;

                if char.is_control() {
                    println!("{:?} \r", byte);
                } else {
                    println!("{:?} ({})\r", byte, char);
                }

                if byte == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => finish_with_error(e),
        }
    }
}
