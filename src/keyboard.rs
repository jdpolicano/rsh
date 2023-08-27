use std::io::{self, Read};

pub struct KeyBoardReader<R> {
    input: R,
}

pub enum ReadResult {
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

    pub fn read_key_press(&mut self) -> ReadResult {
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

    pub fn process_key_press(&mut self) -> Option<u8> {
        match self.read_key_press() {
            ReadResult::Data(byte) => Some(byte),
            ReadResult::NoData => None,
            ReadResult::Error(io_err) => {
                println!("{}", io_err);
                None
            }
        }
    }
}