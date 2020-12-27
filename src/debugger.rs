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
    #[structopt(visible_alias = "c", about = "continue debugging session")]
    Continue,
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

    use nix::{
        sys::{
            ptrace,
            wait::{self, WaitStatus},
        },
        unistd::Pid,
    };

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

        fn enable_target_trace(cmd: &mut Command) {
            unsafe {
                use std::os::unix::process::CommandExt;
                cmd.pre_exec(|| {
                    ptrace::traceme().or(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "ptrace TRACEME failed",
                    )))
                });
            }
        }

        pub fn cont(&mut self) {
            if let Some(ref process) = self.process {
                match ptrace::cont(Pid::from_raw(process.id() as i32), None) {
                    some => println!("Process Continue signalled {:?}", some),
                };
                self.wait();
            } else {
                eprintln!("Error: No process to continue");
            }
        }

        fn wait(&mut self) {
            if let Some(ref process) = self.process {
                let pid = Pid::from_raw(process.id() as i32);
                match wait::waitpid(pid, None) {
                    Ok(WaitStatus::Exited(pid, exit_code)) => {
                        println!("Process {} exited with {}", pid, exit_code);
                        self.process = None
                    }
                    some => println!("Process {} Paused with {:?}", pid, some),
                }
            } else {
                eprintln!("Error: No process to wait");
            }
        }

        pub fn run(&mut self, args: &Vec<String>) {
            self.kill();

            let mut cmd = Command::new(&self.binary);
            cmd.args(args);
            Self::enable_target_trace(&mut cmd);
            self.process = match cmd.spawn() {
                Ok(process) => Some(process),
                Err(err) => {
                    eprintln!("Error: could not start process\n{}", err);
                    None
                }
            };
            self.cont();
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
