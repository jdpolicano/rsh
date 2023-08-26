/// Purpose: Read from Stdin and handle continutation sequences etc...
use std:: {
    io::{self, Write, Read, BufRead}
};

pub struct TermInterface<R, W> {
    reader: R,
    writer: W,
    prompt: String,
    tmp_prompt: String,
    internal_buffer: String,
    in_double_quote: bool,
    in_single_quote: bool,
    in_escape: bool,
}

impl <R: Read + BufRead, W: Write> TermInterface<R, W> {
    pub fn new(reader: R, writer: W, prompt: &str) -> TermInterface<R, W> {
        TermInterface {
            reader,
            writer,
            prompt: prompt.to_string(),
            tmp_prompt: String::new(),
            internal_buffer: String::new(),
            in_single_quote: false,
            in_double_quote: false,
            in_escape: false,
        }
    }

    pub fn read_input(&mut self, buffer: &mut String) -> Result<usize, io::Error> {
        self.writer.write(&self.tmp_prompt.as_bytes())?;
        self.writer.write(&self.prompt.as_bytes())?;
        Write::flush(&mut self.writer)?;

        // for now, this will cause issues if the user attempts to paste to the console, but it
        // is fine in the early stages here.
        let mut tmp_string = String::new();
        let line_count = self.reader.read_line(&mut tmp_string)?;
        
        for c in tmp_string.trim().chars() {
            self.handle_char(c);
        }

        if self.is_terminated() {
            buffer.push_str(&self.internal_buffer);
            self.reset_state();
            Ok(line_count)
        } else {
            self.update_prompt();
            Ok(0)
        }
    }

    fn handle_char(&mut self, c: char) {
        match c {
            // Single quote
            '\'' => {
                if self.in_escape {
                    self.internal_buffer.push(c);
                    self.in_escape = false;
                } else if self.in_double_quote {
                    self.internal_buffer.push(c);
                } else {
                    self.in_single_quote = !self.in_single_quote;
                }
            },
            // double quote
            '\"' => {
                if self.in_escape {
                    self.internal_buffer.push(c);
                    self.in_escape = false;
                } else if self.in_single_quote {
                    self.internal_buffer.push(c);
                } else {
                    self.in_double_quote = !self.in_double_quote;
                }
            },

            // escape character
            '\\' => {
                if self.in_escape {
                    self.internal_buffer.push(c);
                    self.in_escape = false;
                } else {
                    self.in_escape = true;
                }
            }

            // Everything else...
            _ => {
                self.internal_buffer.push(c);
            }
        }
    }

    fn is_terminated(&self) -> bool {
        !self.in_single_quote && !self.in_double_quote && !self.in_escape
    }
    
    fn update_prompt(&mut self) {
        if self.in_double_quote || self.in_escape {
            self.tmp_prompt = "dquote".to_string();
        } else {
            self.tmp_prompt = "quote".to_string();
        }
    }

    fn reset_state(&mut self) {
        self.internal_buffer.clear();
        self.tmp_prompt.clear();
    }
}

