use crate::terminal::Terminal;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Error initializing Terminal instance"),
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.render_editor() {
                Editor::quit_with_error(&error);
            }

            if let Err(error) = self.process_keypress() {
                Editor::quit_with_error(&error);
            }

            if self.should_quit {
                break;
            }
        }
    }

    fn quit_with_error(e: &std::io::Error) {
        print!("{}", termion::clear::All);
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

    fn render_editor(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows()
        }

        stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
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
