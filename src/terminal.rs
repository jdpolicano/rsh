use std::io::{self, BufWriter, Write, Read};
use std::collections::VecDeque;
use crate::keyboard::KeyBoardReader;
use crate::keystroke::{KeyPress, KeyType};
use crate::dimensions::Dimensions;

pub struct Terminal<R: Read, W: Write> {
    reader: KeyBoardReader<R>,
    writer: BufWriter<W>,
    buffer: Vec<VecDeque<char>>,
    should_exit: bool,
    cursor_row: usize,
    cursor_col: usize,
    dimensions: Dimensions,
}

impl<R: Read, W: Write> Terminal<R, W> {
    pub fn new(stdin: R, writer: W, dimensions: Dimensions) -> Self {
        let mut buffer = Vec::with_capacity(dimensions.rows);

        for _ in 0..dimensions.rows {
            buffer.push(VecDeque::with_capacity(dimensions.cols));
        }

        Terminal {
            reader: KeyBoardReader::new(stdin), // wraps basic stdin reader.
            writer: BufWriter::new(writer),
            buffer,
            dimensions,
            cursor_row: 0,
            cursor_col: 0,
            should_exit: false,
        }
    }

    pub fn run(mut self) -> Vec<VecDeque<char>> {
        self.erase_all();
        self.refresh();

        while !self.should_exit {
            match self.reader.read_key() {
                Ok(Some(key)) => self.handle_key_press(key),
                Err(io_err) => {
                    // Handle io errors, for now using a placeholder
                    todo!("Handle io errors....");
                }
                _ => continue,
            }
            self.refresh();
        }

        self.erase_all();
        self.writer.flush();
        self.buffer
    }

    fn handle_key_press(&mut self, key: KeyPress) {
        println!("handling key {}\r", key.get_byte());

        match key.get_type() {
            KeyType::Char => {
                if self.cursor_col == self.dimensions.cols {
                    if self.cursor_row == self.dimensions.rows {
                        self.buffer.remove(0);
                        self.buffer.push(VecDeque::with_capacity(self.dimensions.cols));
                        self.cursor_row = self.dimensions.rows - 1;
                    } else {
                        self.cursor_row += 1;
                    }
                    self.cursor_col = 0;
                }

                self.buffer[self.cursor_row].insert(self.cursor_col, key.get_char());
                self.cursor_col += 1;
            },

            KeyType::EndOfText => {
                self.should_exit = true;
            },

            KeyType::Escape => {
                self.handle_escape_sequence();
            },

            KeyType::Delete => {
                self.handle_delete();
            },

            _ => {
                println!("handling other key {}\r", key.get_byte());
            }
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
                'C' if self.cursor_col < self.buffer[self.cursor_row].len() => {
                    // right arrow
                    self.cursor_col += 1;
                },

                'D' if self.cursor_col > 0 => {
                    // left arrow
                    self.cursor_col -= 1;
                },

                '3' => {
                    // delete key
                    if let Ok(key3) = self.reader.read_key_wait() {
                        if key3.get_char() == '~' {
                            self.handle_delete();
                        }
                    }
                },
                _ => {} // handle other escape sequences if needed
            }
        }
    }

    fn handle_delete(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
            self.buffer[self.cursor_row].remove(self.cursor_col);
        }
    }

    fn refresh(&mut self) {
        self.hide_cursor();
        self.handle_overflow();

        for rc in 0..self.dimensions.rows {
            self.move_cursor(rc as u8 + 1, 1); // first row column 1.
            self.erase_current_row();
            let bytes: Vec<u8> = self.buffer[rc].iter().map(|c| *c as u8).collect();
            self.write_bytes(&bytes);
        }

        self.move_cursor(self.cursor_row as u8 + 1, self.cursor_col as u8 + 1);
        self.show_cursor();
        self.writer.flush();
    }

    // should resize the buffer so that any rows greater than the current width/height 
    // overflow into the next row, cutting off at the last row...
    fn handle_overflow(&mut self) {
        // for each row in the buffer, if the row is greater than the current width, 
        // then we need to resize the row to the current width, and then move the extra elements
        // the next row, if the next row is full, do the same...
        for row in 0..self.buffer.len() - 1 {
            while self.buffer[row].len() > self.dimensions.cols {
                let last_char = self.buffer[row].pop_back().unwrap();
                self.buffer[row + 1].push_front(last_char);
            }
        }

        let last_row = self.buffer.len() - 1;
        while self.buffer[last_row].len() > self.dimensions.cols {
            self.buffer[last_row].pop_back().unwrap();
        }
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

    fn hide_cursor(&mut self) {
        self.write_bytes("\x1b[?25l".as_bytes());
    }

    fn show_cursor(&mut self) {
        self.write_bytes("\x1b[?25h".as_bytes());
    }
}
