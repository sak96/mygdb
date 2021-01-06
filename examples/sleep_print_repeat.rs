use std::{env::args, process::exit, thread::sleep, time::Duration};

fn main() {
    let args: Vec<_> = args().collect();

    // check length and second argument
    if args.len() != 2 || args[1].parse::<usize>().is_err() {
        eprintln!("Usage {} <seconds to sleep>", args[0]);
        exit(1);
    }

    // sleep print repeat
    let sleep_in_seconds = args[1].parse().unwrap();
    for i in 0..sleep_in_seconds {
        sleep(Duration::from_secs(1));
        println!("{}", i);
    }
}
