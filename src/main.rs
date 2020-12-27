use nix::sys::signal::{signal, SigHandler, Signal};

mod debugger;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <binary of program>", args[0]);
        std::process::exit(1);
    }
    unsafe { signal(Signal::SIGINT, SigHandler::SigIgn) }.expect("Error disabling SIGINT handling");
    crate::debugger::Debugger::new(&args[1]).run();
}
