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
    help_string: String,
}

impl Debugger {
    pub fn new(binary: &str) -> Self {
        let help_string: String = {
            let mut help = std::io::Cursor::new(Vec::new());
            DebugCommand::clap()
                .write_long_help(&mut help)
                .unwrap_or(());
            String::from_utf8(help.into_inner())
                .unwrap_or("count not convert help to utf8".into())
                .splitn(2, "SUBCOMMANDS:\n")
                .nth(1)
                .unwrap_or("could not extra subcommands")
                .into()
        };
        Self {
            target: Target::new(binary),
            interpreter: Interpreter::new(&format!("{}> ", env!("CARGO_PKG_NAME"))),
            help_string,
        }
    }

    pub fn run(&mut self) {
        while let Ok(line) = self.interpreter.read_line() {
            match DebugCommand::from_iter_safe(line.split_whitespace()) {
                Ok(cmd) => match cmd {
                    DebugCommand::Quit => break,
                    DebugCommand::Run { args } => self.target.run(&args),
                    DebugCommand::Continue => self.target.cont(),
                    DebugCommand::Help => println!("{}", self.help_string),
                    DebugCommand::BackTrace => self.target.print_trace(),
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
