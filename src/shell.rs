use crate::engine::{ Engine, EngineCtx };
use crate::parser::{ Parser, ParseError };
use std::io::{ self };
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
        // to-do - handle history and completion
        let mut rl = rustyline::DefaultEditor::new()?;

        while !should_stop {
            let readline = rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    // todo - handle builtins like cd, exit, etc.
                    if line == "exit" {
                        should_stop = true;
                    } else {
                        let mut parser = Parser::new(&line);
                        let root = parser.parse()?;
                        let engine = Engine::new(root);
                        let prog_res = engine.execute();
                        if let Ok(mut prog) = prog_res {
                            let _ = self.handle_prog_result(&mut prog);
                        } else {
                            // handle error
                            println!("Error: {:?}", prog_res);
                        }
                    }
                }
                Err(err) => { 
                    println!("Error: {:?}", err);
                    should_stop = true;
                },
            }
        }
        Ok(())
    }

    fn handle_prog_result(&self, ctx: &mut EngineCtx) -> Result<(), io::Error> {
        if let Some(mut c) = ctx.take_last_child() {
            let child_stdout = c.stdout.take();
            let child_stderr = c.stderr.take();

            if let Some(mut stdout) = child_stdout {
                thread::spawn(move || {
                    let _ = io::copy(&mut stdout, &mut io::stdout());
                });
            }

            if let Some(mut stderr) = child_stderr {
                thread::spawn(move || {
                    let _ = io::copy(&mut stderr, &mut io::stderr());
                });
            }

            // Wait for the child process to finish.
            c.wait()?;
            return Ok(());
        }

        return Ok(());
    }
}