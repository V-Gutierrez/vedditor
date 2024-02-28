use std::io::{stdin, stdout};
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    #[allow(clippy::unused_self)]
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{c:?}\r");
                        } else {
                            println!("{c:?} ({c})\r");
                        }
                    }
                    Key::Ctrl('c') => {
                        println!("{key:?}\r");
                        break;
                    }
                    _ => println!("{key:?}\r"),
                },
                Err(e) => Editor::finish_with_error(&e),
            }
        }
    }

    fn finish_with_error(e: &std::io::Error) {
        panic!("{e}");
    }
}
