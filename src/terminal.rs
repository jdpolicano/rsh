use std::io::{self, Write, Read};
use crate::keyboard::KeyBoardReader;
use crate::keystroke::{KeyPress, KeyType};

pub struct Terminal<R, W> {
    reader: KeyBoardReader<R>,
    writer: W,
    buffer: Vec<char>,
    cursor: usize,
    should_exit: bool,
}

impl<R: Read, W: Write> Terminal<R, W> {
    pub fn new(reader: KeyBoardReader<R>, writer: W) -> Self {
        Terminal {
            reader,
            writer,
            buffer: Vec::new(),
            cursor: 0,
            should_exit: false,
        }
    }

    pub fn run(&mut self) -> usize {
        self.erase_all();
        self.refresh();

        while !self.should_exit {
            match self.reader.read_key() {
                Ok(Some(key)) => self.handle_key_press(key),
                Err(io_err) => {
                    println!("\r{}", io_err);
                    // Handle io errors, for now using a placeholder
                    todo!("Handle io errors....");
                }
                _ => continue,
            }
        }

        self.write_bytes("\r\n".as_bytes());
        self.erase_all();
        0
    }

    fn handle_key_press(&mut self, key: KeyPress) {
        match key.get_type() {
            KeyType::Char => {
                self.buffer.insert(self.cursor, key.get_char());
                self.cursor += 1;
                self.refresh();
            }
            KeyType::EndOfText => {
                self.should_exit = true;
            }
            KeyType::Escape => {
                self.handle_escape_sequence();
            }
            _ => {}
        }
    }

    fn handle_escape_sequence(&mut self) {
        if let Ok(key1) = self.reader.read_key_wait() {
            if key1.get_char() == '[' {
                self.handle_arrow_keys();
            } else {
                self.handle_key_press(key1);
            }
        }
    }

    fn handle_arrow_keys(&mut self) {
        if let Ok(key2) = self.reader.read_key_wait() {
            match key2.get_char() {
                'C' if self.cursor < self.buffer.len() => {
                    // right arrow
                    self.cursor += 1;
                }
                'D' if self.cursor > 0 => {
                    // left arrow
                    self.cursor -= 1;
                }
                _ => {} // handle other escape sequences if needed
            }
            self.refresh();
        }
    }

    fn refresh(&mut self) {
        self.move_cursor(1, 1);
        self.erase_current_row();
        let bytes: Vec<u8> = self.buffer.iter().map(|c| *c as u8).collect();
        self.write_bytes(&bytes);
        self.move_cursor(1, self.cursor as u8 + 1);
        self.writer.flush();
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        if let Err(io_err) = self.writer.write(bytes) {
            println!("{}", io_err);
        }
    }

    fn erase_all(&mut self) {
        self.write_bytes("\x1b[2J".as_bytes());
    }

    fn erase_current_row(&mut self) {
        self.write_bytes("\x1b[K".as_bytes());
    }

    fn move_cursor(&mut self, row: u8, column: u8) {
        self.write_bytes(format!("\x1b[{};{}H", row, column).as_bytes());
    }
}
