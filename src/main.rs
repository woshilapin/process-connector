extern crate getopts;

use std::env;
use getopts::Options;
use process::Process;
mod process;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <botA_cmd> <botB_cmd>", program);
    print!("{}", opts.usage(&brief));
}

fn connect(process_a: &mut Process, process_b: &mut Process) {
    loop {
        match process_a.pop() {
            Some(packet) => {
                println!("{}: {}", process_a.name, packet);
                process_b.push(packet);
            }
            None => {}
        }
        match process_b.pop() {
            Some(packet) => {
                println!("{}: {}", process_b.name, packet);
                process_a.push(packet);
            }
            None => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!("error: {}", e) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let cmd_a = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let cmd_b = if matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut process_a = Process::new("game", cmd_a);
    let mut process_b = Process::new("bot", cmd_b);
    process_a.init();
    process_b.init();
    connect(&mut process_a, &mut process_b);
}
