use std::io::{stdin, stdout};
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                Editor::quit_with_error(&error);
            }

            if self.should_quit {
                break;
            }
        }
    }

    fn quit_with_error(e: &std::io::Error) {
        panic!("{e}");
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // ? Stands for ->  If there’s an error, return it, if not, unwrap the value and continue.
        let pressed_key = Editor::read_key()?;

        match pressed_key {
            Key::Ctrl('x') => Ok({
                self.should_quit = true;
            }),
            _ => Ok(()),
        }
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
