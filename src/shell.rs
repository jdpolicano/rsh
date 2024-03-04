use crate::engine::{ Engine, EngineOutput };
use crate::parser::{ Parser, ParseError };
use std::io::{ self, Write };
use rustyline;

#[derive(Debug)]
pub enum RshError {
    ParseError(ParseError),
    ReadlineError(rustyline::error::ReadlineError),
    IoError(io::Error),
}

impl From<ParseError> for RshError {
    fn from(err: ParseError) -> RshError {
        RshError::ParseError(err)
    }
}

impl From<rustyline::error::ReadlineError> for RshError {
    fn from(err: rustyline::error::ReadlineError) -> RshError {
        RshError::ReadlineError(err)
    }
}

impl From<io::Error> for RshError {
    fn from(err: io::Error) -> RshError {
        RshError::IoError(err)
    }
}

pub struct Rsh {
    prompt: String,
}

impl Rsh {
    pub fn new(prompt: String) -> Rsh {
        Rsh {
            prompt,
        }
    }

    pub fn run(&mut self) -> Result<(), RshError>{
        let mut should_stop = false;
        let mut rl = rustyline::DefaultEditor::new()?;

        while !should_stop {
            let readline = rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    if line == "exit" {
                        should_stop = true;
                    } else {
                        let mut parser = Parser::new(&line);
                        let root = parser.parse()?;
                        let engine = Engine::new(root);
                        let proc = engine.execute();
                        self.handle_proc_result(proc);
                    }
                }
                Err(err) => { 
                    println!("{:?}", err);
                    println!("No input given. Exiting...");
                    should_stop = true;
                },
            }
        }
        Ok(())
    }

    fn handle_proc_result(&self, child_result: EngineOutput) {
        match child_result {
            EngineOutput::Single(child_result) => {
                child_result.as_ref().map_err(|e| {
                    println!("{}", e);
                });
                
                if child_result.is_ok() {
                    let child = child_result.unwrap();
                    let result = child.wait_with_output().unwrap();
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stdout().flush().unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    io::stdout().flush().unwrap();
                } 
            },
            EngineOutput::Pipe(left_result, right_result) => {
       
            },
        }
    }
}