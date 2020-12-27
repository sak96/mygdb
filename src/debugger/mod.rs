extern crate structopt;

use debug_command::DebugCommand;
use interpreter::Interpreter;
use structopt::StructOpt;
use target::Target;

mod debug_command;
mod interpreter;
mod target;

pub struct Debugger {
    target: Target,
    interpreter: Interpreter,
}

impl Debugger {
    pub fn new(binary: &str) -> Self {
        Self {
            target: Target::new(binary),
            interpreter: Interpreter::new(&format!("{}> ", env!("CARGO_PKG_NAME"))),
        }
    }
    pub fn run(&mut self) {
        while let Ok(line) = self.interpreter.read_line() {
            match DebugCommand::from_iter_safe(line.split_whitespace()) {
                Ok(cmd) => match cmd {
                    DebugCommand::Quit => break,
                    DebugCommand::Run { args } => self.target.run(&args),
                    DebugCommand::Continue => self.target.cont(),
                    DebugCommand::Help => DebugCommand::clap()
                        .after_help("")
                        .print_long_help()
                        .unwrap_or(()),
                },
                Err(err) => {
                    if !line.trim().is_empty() {
                        eprintln!(
                            "Error: {}",
                            err.to_string()
                                .lines()
                                .nth(0)
                                .unwrap_or("something went wrong!")
                        )
                    }
                }
            };
        }
    }
}
