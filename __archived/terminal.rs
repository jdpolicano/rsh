use std::io::{Write, Read};
use std::os::fd::{AsRawFd, RawFd};
use crate::keyboard::KeyBoardReader;
use crate::linefeed::{LineFeed, ComponentState};
use crate::dimensions::Dimensions;
use crate::environment::EnvBuilder;


pub struct Terminal<R: Read, W: Write> {
    fd: RawFd,
    reader: KeyBoardReader<R>,
    component: LineFeed<W>,
    should_exit: bool,
}

impl<R: Read + AsRawFd, W: Write + AsRawFd> Terminal<R, W> {
    pub fn new(stdin: R, writer: W) -> Self {
        Terminal {
            fd: stdin.as_raw_fd(),
            reader: KeyBoardReader::new(stdin), // wraps basic stdin reader.
            component: LineFeed::new(writer, dimensions),
            should_exit: false,
        }
    }

    pub fn run(mut self) {
        if let Ok(builder) = EnvBuilder::new(self.fd) {
            let term_env = builder
                .enable_raw_mode()
                .set_vmin(0)
                .set_vtime(1)
                .set_env()
                .unwrap();

            while !self.should_exit {
                match self.reader.read_key() {
                    Ok(Some(key)) => {
                        match self.component.handle_key_press(key) {
                            ComponentState::Error | ComponentState::Stop => { 
                                self.should_exit = true;
                                let res = self.component.get_buffer_as_str();
                                println!("\n\r{}\r", res);                        
                            },

                            _ => { self.should_exit = false; }
                        };
                    },
                    Err(_io_err) => {
                        // Handle io errors, for now using a placeholder
                        todo!("Handle io errors....");
                    }
                    _ => continue,
                }
            }

            term_env.restore().unwrap();
        } 
    }

}