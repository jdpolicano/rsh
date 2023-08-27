/// Purpose: Read from Stdin and Write to Stdout; 
use std:: {
    io::{self, Write, Read}
};

use crate::keyboard::KeyBoardReader;

pub struct Terminal<R, W> {
    reader: KeyBoardReader<R>,
    writer: W
}

impl <R: Read, W: Write> Terminal<R, W> {
    pub fn new(reader: KeyBoardReader<R>, writer: W) -> Terminal<R, W> {
        Terminal {
            reader,
            writer,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.erase_in_display(2); 
            self.move_cursor(0, 0); 
            match self.reader.process_key_press() {
                Some(key) => {
                    if key == self.ctrl(b'c') {
                        break;
                    }
                    self.write_bytes(&[key]);
                    self.writer.flush();
                }

                None => { continue }
            }
        }

        self.erase_in_display(2); 
        self.move_cursor(0, 0);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        if let Err(io_err) = self.writer.write(bytes) {
            println!("{}", io_err);
        }
    }

    pub fn erase_in_display(&mut self, param: u8)  {
        self.write_bytes(format!("\x1b[{}J", param).as_bytes())
    }

    pub fn move_cursor(&mut self, row: u8, column: u8)  {
        self.write_bytes(format!("\x1b[{};{}H", row, column).as_bytes())
    }

    /// Applies a mask over a byte's corresponding ctrl key to retrieve the first 5 bits...
    /// this is liable to be moved elsewhere.
    pub fn ctrl(&self, byte: u8) -> u8 {
        return byte & 0x1f;
    }
}

