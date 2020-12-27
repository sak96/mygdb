extern crate structopt;

use interpreter::Interpreter;
use structopt::{clap::AppSettings, StructOpt};
use target::Target;

pub struct Debugger {
    target: Target,
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
                },
                Err(_) => DebugCommand::clap()
                    .after_help("")
                    .print_long_help()
                    .unwrap_or(()),
            };
        }
    }
}

pub(super) mod target {
    use std::process::Child;
    use std::process::Command;

    use nix::{sys::wait, unistd::Pid};

    pub struct Target {
        binary: String,
        process: Option<Child>,
    }

    impl Drop for Target {
        fn drop(&mut self) {
            self.kill()
        }
    }

    impl Target {
        pub fn new(binary: &str) -> Self {
            Self {
                binary: binary.to_string(),
                process: None,
            }
        }

        pub fn kill(&mut self) {
            if let Some(mut process) = self.process.take() {
                process.kill().unwrap_or(())
            }
        }

        pub fn pid(&self) -> Option<Pid> {
            if let Some(ref process) = self.process {
                Some(Pid::from_raw(process.id() as i32))
            } else {
                None
            }
        }

        pub fn run(&mut self, args: &Vec<String>) {
            self.kill();

            let mut cmd = Command::new(&self.binary);
            cmd.args(args);
            self.process = match cmd.spawn() {
                Ok(process) => Some(process),
                Err(err) => {
                    eprintln!("Error: could not start process\n{}", err);
                    None
                }
            };
            if let Some(pid) = self.pid() {
                match wait::waitpid(pid, None) {
                    some => println!("Wait Status {:?}", some),
                }
            }
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
