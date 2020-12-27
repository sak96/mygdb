extern crate structopt;

use interpreter::Interpreter;
use structopt::{clap::AppSettings, StructOpt};

pub struct Debugger {
    binary: String,
    interpreter: Interpreter,
}

#[derive(StructOpt)]
#[structopt(about, global_settings(&[AppSettings::VersionlessSubcommands, AppSettings::NoBinaryName, AppSettings::DisableHelpFlags, AppSettings::DisableVersion]))]
enum DebugCommand {
    #[structopt(visible_alias = "q", about = "quit debugging session")]
    Quit,
    #[structopt(visible_alias = "r", about = "run program with arguments")]
    Run { args: Vec<String> },
}

impl Debugger {
    pub fn new(binary: &str) -> Self {
        Self {
            binary: binary.to_string(),
            interpreter: Interpreter::new(&format!("{}> ", env!("CARGO_PKG_NAME"))),
        }
    }
    pub fn run(&mut self) {
        while let Ok(line) = self.interpreter.read_line() {
            match DebugCommand::from_iter_safe(line.split_whitespace()) {
                Ok(cmd) => match cmd {
                    DebugCommand::Quit => break,
                    DebugCommand::Run { args } => {
                        println!("run {} with args {:?}", self.binary, args)
                    }
                },
                Err(_) => DebugCommand::clap()
                    .after_help("")
                    .print_long_help()
                    .unwrap_or(()),
            };
        }
    }
}

pub(super) mod interpreter {
    use rustyline::{error::ReadlineError, Editor};

    pub struct Interpreter {
        prompt: String,
        reader: Editor<()>,
    }

    impl Interpreter {
        pub fn new(prompt: &str) -> Self {
            let reader = Editor::<()>::new();
            Self {
                prompt: prompt.to_string(),
                reader,
            }
        }

        pub fn read_line(&mut self) -> Result<String, ReadlineError> {
            match self.reader.readline(self.prompt.as_str()) {
                Ok(line) => {
                    self.reader.add_history_entry(&line);
                    Ok(line)
                }
                err => err,
            }
        }
    }
}
