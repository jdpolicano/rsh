use crate::parser::{ RshNode, RedirectMode };
use std::process::{ self };
use std::io::{ self };


pub enum EngineOutput {
    Single(io::Result<process::Child>),
    Pipe(io::Result<process::Child>, io::Result<process::Child>),
}

// This module is the engine that takes a syntax tree and executes it.
// it handles passing along io etc and any threading that needs to happen on the shell side. 
// The "shell" in shell.rs is the orchestrator that manages startup and state of the program throughout. i.e., history, current working directory, etc.
pub struct Engine {
    root: RshNode,
}

impl Engine {
    pub fn new(root: RshNode) -> Engine {
        Engine {
            root: root,
        }
    }

    pub fn execute(&self) -> EngineOutput {
        self.execute_node(&self.root, None)
    }

    fn execute_node(&self, root: &RshNode, input: Option<EngineOutput>) -> EngineOutput {
        match root {
            RshNode::Command { name, args } => {
                self.execute_standard_command(name, args, input)
            },
            RshNode::Pipe { left, right } => {
                let left_output = self.execute_node(left, input); // handle if this returns a single (proc) or an (in, out) proc...
                let left_out_channel = self.take_child(left_output, true);
                let right_output = self.execute_node(right, Some(EngineOutput::Single(left_out_channel))); // handle if this returns a single (proc) or an (in, out) proc...
                let right_out_channel = self.take_child(right_output, true);
                EngineOutput::Single(right_out_channel)
            },
            _ => { todo!("implement the rest of the node types"); }
            // RshNode::Redirect { command, file, mode } => {
            //     let mut command = Command::new(command.name);
            //     for arg in command.args {
            //         command.arg(arg);
            //     }
            //     match mode {
            //         RedirectMode::Read => {
            //             let file = std::fs::File::open(file).expect("failed to open file");
            //             command.stdin(file);
            //         },
            //         RedirectMode::Write => {
            //             let file = std::fs::File::create(file).expect("failed to create file");
            //             command.stdout(file);
            //         },
            //         RedirectMode::Append => {
            //             let file = std::fs::OpenOptions::new().append(true).open(file).expect("failed to open file");
            //             command.stdout(file);
            //         },
            //     }
            //     command.spawn().expect("failed to execute process");
            // },
            // RshNode::Background { command } => {
            //     let mut command = Command::new(command.name);
            //     for arg in command.args {
            //         command.arg(arg);
            //     }
            //     command.spawn().expect("failed to execute process");
            // },
        }
    }

    fn execute_standard_command(&self, name: &str, args: &[String], input: Option<EngineOutput>) -> EngineOutput {
        let mut command = process::Command::new(name);
        for arg in args {
            command.arg(arg);
        }

        if let Some(target_input) = input {
            let input_channel_result = self.take_child(target_input, true);
            if input_channel_result.is_ok() {
                let input_channel = input_channel_result.unwrap();
                command.stdin(input_channel.stdout.expect("Failed to capture stdout"));
                command.stdout(process::Stdio::piped());
                command.stderr(process::Stdio::piped());
                return EngineOutput::Single(command.spawn())
            };
        };
        
        command.stdin(process::Stdio::piped());
        command.stdout(process::Stdio::piped());
        command.stderr(process::Stdio::piped());
        // spawn and wait for the process to finish.
        EngineOutput::Single(command.spawn())
    }

    fn take_child(&self, engine_out: EngineOutput, readable: bool) -> io::Result<process::Child> {
        match engine_out {
            EngineOutput::Single(child) => child,
            EngineOutput::Pipe(child_in, child_out) => if readable { child_out } else { child_in }
        }
    }
}