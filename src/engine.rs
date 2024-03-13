use crate::parser::{ RshNode, RedirectMode };
use std::process::{ Child, ChildStdout, Command, Stdio };
use std::io::{ self };
use std::fs::{ File, OpenOptions };

/**
* The context that the engine will use to execute commands. Keeps track of the number of commands executed
keeps refs to all of the children, and other useful information.
*/
#[derive(Debug)]
pub struct EngineCtx {
    command_count: u32,
    should_pipe: bool,
    pub children: Vec<Child>,
}

impl EngineCtx {
    pub fn new() -> EngineCtx {
        EngineCtx {
            command_count: 0,
            should_pipe: false,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Child) {
        self.children.push(child);
        self.command_count += 1;
    }

    pub fn get_child(&mut self, index: usize) -> Option<&Child> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Child> {
        self.children.get_mut(index)
    }

    pub fn get_last_child(&mut self) -> Option<&Child> {
        self.children.last()
    }

    pub fn get_last_child_mut(&mut self) -> Option<&mut Child> {
        self.children.last_mut()
    }

    pub fn take_last_child(&mut self) -> Option<Child> {
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

    pub fn execute(&self) -> Result<EngineCtx, io::Error> {
        let mut ctx = EngineCtx::new();
        self.execute_node(&self.root, &mut ctx)?;
        Ok(ctx)
    }

    fn execute_node(&self, root: &RshNode, ctx: &mut EngineCtx) -> Result<(), io::Error> {
        match root {
            RshNode::Command { name, args } => {
                if ctx.should_pipe() {
                    self.execute_cmd_pipe(name, args, ctx)?;
                } else {
                    self.execute_cmd_std(name, args, ctx)?;
                }
            },

            RshNode::Redirect { command, file, mode } => {
                let name = command.get_name().unwrap();
                let args = command.get_args().unwrap();
                self.execute_cmd_redir(name, args, file, mode, ctx)?; 
            },
            RshNode::Pipe { left, right } => {
                self.execute_node(left, ctx)?;
                ctx.set_pipe(true);
                self.execute_node(right, ctx)?;
                ctx.set_pipe(false);
            },
            _ => { todo!("implement the rest of the node types"); }
        }
        Ok(())
    }

    fn execute_cmd_std(&self, name: &str, args: &[String], ctx: &mut EngineCtx) -> Result<(), io::Error> {
        let mut command = self.setup_command(name, args); 
        self.setup_io::<Stdio>(&mut command, Some(Stdio::inherit()), None, None);
        let child = command.spawn()?;
        ctx.add_child(child);
        Ok(())
    }


    fn execute_cmd_redir(&self, name: &str, args: &[String], file: &str, mode: &RedirectMode, ctx: &mut EngineCtx) -> Result<(), io::Error> {
        let mut command = self.setup_command(name, args); 

        match mode {
            RedirectMode::Read => {
                let file = File::open(file).expect("failed to open file");
                self.setup_io::<File>(&mut command, Some(file), None, None);
            },
            RedirectMode::Write => {
                let file = File::create(file).expect("failed to create file");
                self.setup_io::<File>(&mut command, None, Some(file), None);
            },
            RedirectMode::Append => {
                let file = OpenOptions::new().append(true).open(file).expect("failed to open file");
                self.setup_io::<File>(&mut command, Some(file), None, None);
            },
        }

        let child = command.spawn()?;
        ctx.add_child(child);
        Ok(())
    }

    fn execute_cmd_pipe(&self, name: &str, args: &[String], ctx: &mut EngineCtx) -> Result<(), io::Error> {
        let mut command = self.setup_command(name, args); 

        if let Some(last_child) = ctx.get_last_child_mut() {
            let stdout = last_child.stdout.take().unwrap();
            self.setup_io::<ChildStdout>(&mut command, Some(stdout), None, None);
            let piped_child = command.spawn()?;
            ctx.add_child(piped_child);
            return Ok(())
        }

        // todo handle this gracefully...
        unreachable!()
    }

    fn setup_command(&self, name: &str, args: &[String]) -> Command {
        let mut command = Command::new(name);
        for arg in args {
            command.arg(arg);
        }
        command
    }

    fn setup_io<T: Into<Stdio>>(&self, command: &mut Command, stdin: Option<T>, stdout: Option<T>, stderr: Option<T>) {
        if let Some(stdin) = stdin { 
            command.stdin(stdin); 
        } else { 
            command.stdin(Stdio::piped()); 
        }

        if let Some(stdout) = stdout { 
            command.stdout(stdout); 
        } else { 
            command.stdout(Stdio::piped()); 
        }

        if let Some(stderr) = stderr { 
            command.stderr(stderr); 
        } else { 
            command.stderr(Stdio::piped()); 
        }
    }
}