use std::io::{stdin, stdout};
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('c') => {
                        println!("{:?}\r", key);
                        break;
                    }
                    _ => println!("{:?}\r", key),
                },
                Err(e) => Editor::finish_with_error(e),
            }
        }
    }

    fn finish_with_error(e: std::io::Error) {
        panic!("{}", e);
    }
}
