use std::io::{self, Read};
use crate::keystroke::{KeyPress};

pub struct KeyBoardReader<R> {
    input: R,
}

enum ReadResult {
    Data(u8),
    NoData,
    Error(io::Error),
}

impl <R: Read> KeyBoardReader <R> {
    pub fn new(reader: R) -> KeyBoardReader<R> {
        KeyBoardReader {
            input: reader,
        }
    }

    fn read_key_press(&mut self) -> ReadResult {
        let mut c = [0];
        match self.input.read(&mut c) {
            Ok(num_bytes) => {
                if num_bytes > 0 {
                    ReadResult::Data(c[0])
                } else {
                    ReadResult::NoData
                }
            },
            Err(io_err) => ReadResult::Error(io_err)
        }
    }

    // non-blocking read for one char, can return none...tranfroms it into a keypress.
    pub fn read_key(&mut self) -> Result<Option<KeyPress>, io::Error> {
        match self.read_key_press() {
            ReadResult::Data(byte) => {
                Ok(Some(KeyPress::new(byte)))
            },
            ReadResult::NoData => { 
                Ok(None)
            },
            ReadResult::Error(io_err) => {
                println!("{}", io_err);
                Err(io_err)
            }
        }
    }

    // Blocking version of read key...
    /// to-do handle errors...
    pub fn read_key_wait(&mut self) -> Result<KeyPress, io::Error> {
        loop {
            match self.read_key_press() {
                ReadResult::Data(byte) => {
                    return Ok(KeyPress::new(byte));
                },
                ReadResult::NoData => { continue },
                ReadResult::Error(io_err) => {
                    println!("{}", io_err);
                    return Err(io_err);
                }
            }
        }
    } 
}