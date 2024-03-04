use crate::keystroke::{InputType, AsciiKey, EscapeSequence};
use std::io::{self, Read};
use std::time::{Duration, Instant};

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

    /*
    Reads a sinle key from stdin and returns it as a read result. 
    This is a non-blocking read, so it will return immediately.
    */
    fn read_key_press(&mut self) -> ReadResult {
        let mut buf: [u8; 1] = [0; 1];

        match self.input.read(&mut buf) {
            Ok(num_bytes) => {
                if num_bytes > 0 {
                    ReadResult::Data(buf[0])
                } else {
                    ReadResult::NoData
                }
            },
            Err(io_err) => ReadResult::Error(io_err)
        }
    }

    // non-blocking read for one char, can return none...tranfroms it into a keypress.
    // If key[0] is an escape key, this will trigger a time-based read. It will consume
    // as many bytes as possible in the time given (1/10 of a second) and try to convert it
    // to an escape sequence...if it can't, it will return an escape key. 
    pub fn read_key(&mut self) -> Result<Option<InputType>, io::Error> {
        match self.read_key_press() {
            ReadResult::Data(byte) => {
                let key = AsciiKey::new(&[byte]);
                if key == AsciiKey::Escape {
                    self.read_escape_sequence()
                } else {
                    Ok(Some(InputType::Ascii(key)))
                }
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

    // Will read as many bytes as possible in 1/10 of a second and try to convert it to an escape sequence.
    // Will return none if the escape sequence variant is NoOp.
    fn read_escape_sequence(&mut self) -> Result<Option<InputType>, io::Error> {
        let mut buffer = Vec::with_capacity(8); // no read for an escape should be greater than this I think?
        buffer.push(0x1b); // push the escape key

        let mut bytes_read = 0;
        let start = Instant::now();
        let timeout = Duration::from_millis(100); // 1/10 of a second

        while Instant::now() - timeout < start && bytes_read < buffer.len() {
            match self.read_key_press() {
                ReadResult::Data(byte) => {
                    buffer.push(byte);
                    bytes_read += 1;
                    let esc = EscapeSequence::new(&buffer);
                    // if it the enum variant is not no-op then return it.
                    if esc.is_valid() {
                        return Ok(Some(InputType::Ansi(esc)));
                    }
                },

                ReadResult::NoData => {
                    break;
                },

                ReadResult::Error(io_err) => {
                    println!("{}", io_err);
                    return Err(io_err);
                }
            }
        }
        Ok(None)
    }
}