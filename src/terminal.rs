use std::io::{self, BufWriter, Write, Read};
use std::collections::VecDeque;
use crate::keyboard::KeyBoardReader;
use crate::linefeed::{LineFeed, ComponentState};
use crate::dimensions::Dimensions;

pub struct Terminal<R: Read, W: Write> {
    reader: KeyBoardReader<R>,
    component: LineFeed<W>,
    should_exit: bool,
}

impl<R: Read, W: Write> Terminal<R, W> {
    pub fn new(stdin: R, writer: W, dimensions: Dimensions) -> Self {
        Terminal {
            reader: KeyBoardReader::new(stdin), // wraps basic stdin reader.
            component: LineFeed::new(writer, dimensions),
            should_exit: false,
        }
    }

    pub fn read_line(mut self) {
        while !self.should_exit {
            match self.reader.read_key() {
                Ok(Some(key)) => {
                    match self.component.handle_key_press(key) {
                        ComponentState::Error | ComponentState::Stop => { 
                            self.should_exit = true;
                        },

                        _ => { self.should_exit = false; }
                    };
                },
                Err(io_err) => {
                    // Handle io errors, for now using a placeholder
                    todo!("Handle io errors....");
                }
                _ => continue,
            }
        }
    }
}