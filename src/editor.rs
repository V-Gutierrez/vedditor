use crate::terminal::Terminal;
use crate::document::Document;
use crate::Row;
use std::io::stdout;
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const EDITOR_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::init().expect("Error initializing Terminal instance"),
            cursor_position: Position::default(),
            document: Document::open(),
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
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
        Terminal::clear_screen();
        panic!("{e}");
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // ? Stands for ->  If there’s an error, return it, if not, unwrap the value and continue.
        let pressed_key = Terminal::read_key()?;

        match pressed_key {
            Key::Ctrl('x') => Ok({
                self.should_quit = true;
            }),
            Key::Up | Key::Down | Key::Left | Key::Right | Key::PageUp | Key::PageDown | Key::End | Key::Home => Ok(self.move_cursor(pressed_key)),
            _ => Ok(()),
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let authors = AUTHORS.replace(":", " and ");
        let mut welcome_message = format!("{EDITOR_NAME} -- version {VERSION}\n by {authors}");
        let width = self.terminal.size().width as usize;

        let len = welcome_message.len();
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}{spaces}~");
        welcome_message.truncate(width);

        println!("{welcome_message}\r");
    }

    pub fn draw_row(&self, row: &Row){
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);

        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            }
            else if terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("\r");
            }
        }
    }
}
