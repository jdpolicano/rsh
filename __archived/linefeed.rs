use std::io::{BufWriter, Write};
use std::collections::VecDeque;
use crate::keystroke::{InputType, AsciiKey, EscapeSequence};
use crate::dimensions::Dimensions;

/*
Represents the state of a component, whether it should continue, stop, or error.
This will likely get used in the future for other programs, but for now it's just lives here.
*/
pub enum ComponentState {
    Continue,
    Error,
    Stop,
}

/*
LineFeed is a component that handles the logic for a single line of text. It handles user input and collects it internally.
After each key press, it will render the buffer to the screen. 

After compnent recieves command to stop it will send a stop signal to the terminal component as well as the entire contents of the buffer. 
*/
pub struct LineFeed<W: Write> {
    writer: BufWriter<W>,
    buffer: Vec<VecDeque<char>>,
    cursor_row: usize,
    cursor_col: usize,
    dimensions: Dimensions,
}

impl<W: Write> LineFeed<W> {
    pub fn new(writer: W, dimensions: Dimensions) -> Self {
        let mut buffer = Vec::with_capacity(dimensions.rows);

       for _ in 0..dimensions.rows {
            buffer.push(VecDeque::with_capacity(dimensions.cols));
        }

        LineFeed {
            writer: BufWriter::new(writer),
            buffer,
            dimensions,
            cursor_row: 0,
            cursor_col: 0,
        }
    }


    pub fn handle_key_press(&mut self, key: InputType) -> ComponentState {
        let state = match key {
            InputType::Ascii(ascii_key) => {
                self.handle_ascii_key(ascii_key)
            },
            
            InputType::Ansi(escape) => {
                self.handle_escape_sequence(escape)
            },
        };
        
        self.refresh();
        state
    }

    pub fn handle_escape_sequence(&mut self, esc: EscapeSequence) -> ComponentState {
        match esc {
            EscapeSequence::ArrowLeft | EscapeSequence::ArrowRight => {
                self.move_cursor_row_col(esc); 
                ComponentState::Continue
            },

            EscapeSequence::Delete => {
                self.delete_current_position();
                ComponentState::Continue
            },

            _ => {
                ComponentState::Continue
            }
        }
    }
    /*
    Should get the current buffer as a contiguous slice of chars.
    */
    pub fn get_buffer(&self) -> Vec<char> {
        let mut buffer = Vec::with_capacity(self.dimensions.rows * self.dimensions.cols);
        for row in &self.buffer {
            for col in row {
                buffer.push(*col);
            }
        };

        buffer
    }
    /*
    Should get the current buffer as a contiguous slice of chars.
    */
    pub fn get_buffer_as_str(&self) -> String {
        let mut buffer_str = String::with_capacity(self.dimensions.rows * self.dimensions.cols);

        for row in &self.buffer {
            for col in row {
                buffer_str.push(*col);
            }
        };

        buffer_str
    }


    fn handle_ascii_key(&mut self, key: AsciiKey) -> ComponentState {
        match key {
            AsciiKey::Char(c) => {
                // insert the char into the current row/col and increment col and row.
                self.buffer[self.cursor_row].insert(self.cursor_col, c);
                self.cursor_col = (self.cursor_col + 1) % self.dimensions.cols;
                if self.cursor_col == 0 {
                    self.cursor_row = (self.cursor_row + 1) % self.dimensions.rows;
                }
                ComponentState::Continue
            },

            AsciiKey::Delete => {
                self.delete_current_position();
                ComponentState::Continue
            },

            AsciiKey::EndOfText => {
                ComponentState::Stop
            },

            _ => ComponentState::Continue,
        }
    }

    fn move_cursor_row_col(&mut self, key: EscapeSequence) {
        match key {
            EscapeSequence::ArrowLeft => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            },

            EscapeSequence::ArrowRight => {
                if self.cursor_col < self.buffer[self.cursor_row].len() {
                    self.cursor_col = (self.cursor_col + 1) % self.dimensions.cols;
                    if self.cursor_col == 0 {
                        self.cursor_row = (self.cursor_row + 1) % self.dimensions.rows;
                    }
                }
            },

            _ => {}
        }
    }

    fn delete_current_position(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
            self.buffer[self.cursor_row].remove(self.cursor_col);
        }
    }

    fn refresh(&mut self) {
        self.resize_buffer();
        self.hide_cursor();
        self.render();
        self.move_cursor(self.cursor_row as u8 + 1, self.cursor_col as u8 + 1);
        self.show_cursor();
        self.writer.flush().unwrap();
    }

    /*
    Normalize and handles overflow when the text is longer than the terminal width on insert. 
    */
    fn resize_buffer(&mut self) {
        for row in 0..self.dimensions.rows - 1 {
            let row_len = self.buffer[row].len();
            if row_len > self.dimensions.cols {
                let overflow = self.buffer[row].split_off(self.dimensions.cols);
                for el in overflow {
                    self.buffer[row + 1].push_front(el);
                }
            }
        }
    }


    // should render the buffer to the screen.
    fn render(&mut self) {
        for i in 0..self.dimensions.rows {
            if self.buffer[i].len() == 0 {
                break;
            }

            self.move_cursor(i as u8 + 1, 1); // first row column 1.
            self.erase_current_row();
            let bytes: Vec<u8> = self.buffer[i].iter().map(|c| *c as u8).collect();
            self.write_bytes(&bytes);
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        if let Err(io_err) = self.writer.write(bytes) {
            println!("{}", io_err);
        }
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
