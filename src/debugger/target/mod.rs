mod debug_data;

use debug_data::DebugData;
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
    debug_data: DebugData,
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
            debug_data: DebugData::new(binary),
        }
    }

    pub fn kill(&mut self) {
        if let Some(ref mut process) = self.process {
            process.kill().unwrap_or(());
            self.wait();
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
            if let Err(err) = ptrace::cont(Pid::from_raw(process.id() as i32), None) {
                eprintln!("Error: Process could not be continued\n{:?}", err);
            } else {
                self.wait();
            }
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
                Ok(WaitStatus::Stopped(pid, signal)) => {
                    let register = ptrace::getregs(pid).unwrap();
                    let location = self.debug_data.find_location(register.rip);
                    println!(
                        "Process {} stopped (signal {:?}) at {}:{}",
                        pid,
                        signal,
                        location.file.unwrap_or("???"),
                        location.line.map(|k| k).unwrap_or(0)
                    );
                }
                some => println!("Process {} paused with {:?}", pid, some),
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
