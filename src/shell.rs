use crate::engine::{ Engine, EngineCtx, EngineOutput };
use crate::parser::{ Parser, ParseError };
use std::process;
use std::io::{ self, Write, BufRead, BufReader };
use std::thread;
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
                        let mut prog = engine.execute();
                        self.handle_prog_result(&mut prog);
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

    fn handle_prog_result(&self, ctx: &mut EngineCtx) -> Result<(), io::Error> {
        if let Some(mut c) = ctx.take_last_child() {

            let stdout = c.stdout.take().expect("child did not have a handle to stdout");

            let reader = BufReader::new(stdout);
        
            // Use a separate thread to read the child's stdout
            thread::spawn(move || {
                reader.lines().for_each(|line| {
                    if let Ok(l) = line {
                        println!("{}", l);
                    }
                });
            });
        
            // Wait for the child process to finish.
            c.wait()?;
            return Ok(());
        }

        return Ok(());
    }
}