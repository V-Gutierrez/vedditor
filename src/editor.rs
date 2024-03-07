use crate::document::Document;
use crate::terminal::Terminal;
use crate::Row;
use std::env;
use std::io::{stdout, Error};
use std::time::{Duration, Instant};
use termion::color;
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const EDITOR_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const STATUS_BG_COLOR: color::Cyan = color::Cyan;
const STATUS_FG_COLOR: color::Black = color::Black;
const QUIT_TIMES: u8 = 3;

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SearchDirection {
    Forward,
    Backward,
}

pub struct StatusMessage {
    text: String,
    time: Instant,
}

impl From<String> for StatusMessage {
    fn from(text: String) -> Self {
        Self {
            text,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("Ctrl-F = find | Ctrl-S = save | Ctrl-X = quit");

        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);

            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("Error: Could not open file: {file_name}");
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::init().expect("Error initializing Terminal instance"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
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

    fn quit_with_error(e: &Error) {
        Terminal::clear_screen();
        panic!("{e}");
    }

    fn search(&mut self) {
        let old_cursor_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;

        let query = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor: &mut Editor, key: Key, query: &String| {
                    let mut moved = false;

                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        }
                        Key::Left | Key::Up => direction = SearchDirection::Backward,
                        _ => (),
                    };

                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                },
            )
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_cursor_position;
            self.scroll();
        }
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        // ? Stands for ->  If there’s an error, return it, if not, unwrap the value and continue.
        let pressed_key = Terminal::read_key()?;

        match pressed_key {
            Key::Ctrl('x') => Ok({
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING!!! File has unsaved changes. Press Ctrl-X {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;

                    return Ok(());
                }
                self.should_quit = true;
            }),
            Key::Ctrl('s') => {
                self.save();
                Ok(())
            }
            Key::Ctrl('f') => {
                self.search();

                Ok(())
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
                self.reset_quit_times();

                Ok(())
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => {
                self.move_cursor(pressed_key);
                self.scroll();
                self.reset_quit_times();

                Ok(())
            }
            Key::Delete => {
                self.document.delete(&self.cursor_position);
                Ok(())
            }
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1)
        }

        if x < offset.x {
            offset.x = x
        } else if x >= offset.x.saturating_sub(width) {
            offset.x = x.saturating_sub(width).saturating_add(1)
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let terminal_height = self.terminal.size().height as usize;
        let height = self.document.len();

        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;

                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height
                } else {
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let authors = AUTHORS.replace(':', " and ");
        let mut welcome_message = format!("{EDITOR_NAME} -- version {VERSION}\n by {authors}");
        let width = self.terminal.size().width as usize;

        let len = welcome_message.len();
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}{spaces}~");
        welcome_message.truncate(width);

        println!("{welcome_message}\r");
    }

    pub fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let end = self.offset.x + width;

        let row = row.render(start, end);

        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            "(modified)"
        } else {
            ""
        };
        let mut file_name = "[No Name]".to_string();

        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        let mut status = format!(
            "{file_name} - {} lines {modified_indicator}",
            self.document.len()
        );

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let len = line_indicator.len() + status.len();

        if width > len {
            status.push_str(&" ".repeat(width - len));
        }

        status = format!("{status}{line_indicator}");
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();

        let message = &self.status_message;

        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();

            text.truncate(self.terminal.size().width as usize);
            print!("{text}")
        }
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();

        loop {
            self.status_message = StatusMessage::from(format!("{prompt} {result}"));

            self.refresh_screen()?;

            let key = Terminal::read_key()?;

            match key {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c)
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }

            callback(self, key, &result);
        }

        self.status_message = StatusMessage::from(String::new());

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);

            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted".to_string());
                return;
            }

            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully".to_string());
        } else {
            self.status_message = StatusMessage::from("Error saving file".to_string());
        }
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
    }
}
