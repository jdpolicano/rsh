use crate::parser::{ RshNode, RedirectMode };
use std::process::{ self };
use std::io::{ self };
use std::fs::{ File, OpenOptions };
use std::os::fd::AsFd;

pub enum EngineOutput {
    Single(io::Result<process::Child>),
    Pipe(io::Result<process::Child>, io::Result<process::Child>),
}


/**
* The context that the engine will use to execute commands. Keeps track of the number of commands executed
keeps refs to all of the children, and other useful information.
*/
#[derive(Debug)]
pub struct EngineCtx {
    command_count: u32,
    should_pipe: bool,
    children: Vec<process::Child>,
}

impl EngineCtx {
    pub fn new() -> EngineCtx {
        EngineCtx {
            command_count: 0,
            should_pipe: false,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: process::Child) {
        self.children.push(child);
        self.command_count += 1;
    }

    pub fn get_child(&mut self, index: usize) -> Option<&process::Child> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut process::Child> {
        self.children.get_mut(index)
    }

    pub fn get_last_child(&mut self) -> Option<&process::Child> {
        self.children.last()
    }

    pub fn get_last_child_mut(&mut self) -> Option<&mut process::Child> {
        self.children.last_mut()
    }

    pub fn take_last_child(&mut self) -> Option<process::Child> {
        self.children.pop()
    }

    pub fn should_pipe(&self) -> bool {
        self.should_pipe
    }

    pub fn set_pipe(&mut self, should_pipe: bool) {
        self.should_pipe = should_pipe;
    }
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

    pub fn execute(&self) -> EngineCtx{
        let mut ctx = EngineCtx::new();
        self.execute_node(&self.root, &mut ctx);
        ctx
    }

    fn execute_node(&self, root: &RshNode, ctx: &mut EngineCtx) {
        match root {
            RshNode::Command { name, args } => {
                if ctx.should_pipe() {
                    self.execute_cmd_pipe(name, args, ctx);
                } else {
                    self.execute_cmd_std(name, args, ctx);
                }
            },

            RshNode::Redirect { command, file, mode } => {
                let name = command.get_name().unwrap();
                let args = command.get_args().unwrap();
                self.execute_cmd_redir(name, args, file, mode, ctx); 
            },
            RshNode::Pipe { left, right } => {
                self.execute_node(left, ctx);
                ctx.set_pipe(true);
                self.execute_node(right, ctx);
                ctx.set_pipe(false);
            },
            _ => { todo!("implement the rest of the node types"); }
        }
    }

    fn execute_cmd_std(&self, name: &str, args: &[String], ctx: &mut EngineCtx) {
        let mut command = process::Command::new(name);
        for arg in args {
            command.arg(arg);
        }

        command.stdin(process::Stdio::piped());
        command.stdout(process::Stdio::piped());
        command.stderr(process::Stdio::piped());
        // spawn and wait for the process to finish.
        // todo: handle the result of this failure, perhaps have an error message and stuff added to context to repoirt to the shell later?
        let child = command.spawn().expect("failed to execute process");
        // spawn and add the child to the context struct for tracking...
        ctx.add_child(child);
    }


    fn execute_cmd_redir(&self, name: &str, args: &[String], file: &str, mode: &RedirectMode, ctx: &mut EngineCtx) {
        let mut command = process::Command::new(name);
        for arg in args {
            command.arg(arg);
        }

        match mode {
            RedirectMode::Read => {
                let file = File::open(file).expect("failed to open file");
                command.stdin(file);
                command.stdout(process::Stdio::piped());
                command.stderr(process::Stdio::piped());
            },
            RedirectMode::Write => {
                let file = File::create(file).expect("failed to create file");
                command.stdout(file);
                command.stdin(process::Stdio::piped());
                command.stderr(process::Stdio::piped());
            },
            RedirectMode::Append => {
                let file = OpenOptions::new().append(true).open(file).expect("failed to open file");
                command.stdout(file);
                command.stdin(process::Stdio::piped());
                command.stderr(process::Stdio::piped());
            },
        }

        // spawn and add to the ctx obj...
        let child = command.spawn().expect("failed to execute process");
        ctx.add_child(child);
    }

    fn execute_cmd_pipe(&self, name: &str, args: &[String], ctx: &mut EngineCtx) {
        let mut command = process::Command::new(name);
        for arg in args {
            command.arg(arg);
        }

        // is it okay to lose the last_child here? it will no longer be in the list of commands we originally executed...
        let last_child = ctx.take_last_child().unwrap();

        command.stdin(last_child.stdout.unwrap());
        command.stdout(process::Stdio::piped());
        command.stderr(process::Stdio::piped());
        // spawn and wait for the process to finish.
        let piped_child = command.spawn().expect("failed to execute process");
        ctx.add_child(piped_child);
    }
}