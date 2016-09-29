extern crate getopts;

use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, Command, Stdio};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <botA_cmd> <botB_cmd>", program);
    print!("{}", opts.usage(&brief));
}
fn spawn_from_cmd(cmd: String) -> Child {
    let cmd_array: Vec<&str> = cmd.split(" ").collect();
    let child_result = Command::new(cmd_array[0])
        .args(&cmd_array[1..])
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn();
    match child_result {
        Ok(child) => child,
        Err(e) => panic!("error: bot a: {}", e),
    }
}

fn connect(bot_a_child: Child, bot_b_child: Child) {
    let bot_a_stdin = match bot_a_child.stdin {
        Some(value) => value,
        None => panic!("error: bot a: cannot access stdin"),
    };
    let bot_a_stdout = match bot_a_child.stdout {
        Some(value) => value,
        None => panic!("error: bot a: cannot access stdout"),
    };
    let bot_b_stdin = match bot_b_child.stdin {
        Some(value) => value,
        None => panic!("error: bot b: cannot access stdin"),
    };
    let bot_b_stdout = match bot_b_child.stdout {
        Some(value) => value,
        None => panic!("error: bot b: cannot access stdout"),
    };
    let mut bot_a_reader = BufReader::new(bot_a_stdout);
    let mut bot_a_writer = BufWriter::new(bot_a_stdin);
    let mut bot_b_writer = BufWriter::new(bot_b_stdin);
    let mut bot_b_reader = BufReader::new(bot_b_stdout);
    
    loop {
        let mut stdout = std::io::stdout();
        let mut bot_a_line = String::new();
        if bot_a_reader.read_line(&mut bot_a_line).unwrap() > 0 {
            bot_b_writer.write_fmt(format_args!("{}", bot_a_line)).unwrap();
            bot_b_writer.flush().unwrap();
            stdout.write_fmt(format_args!("bot a: {}", bot_a_line)).unwrap();
            stdout.flush().unwrap();
        }
        let mut bot_b_line = String::new();
        if bot_b_reader.read_line(&mut bot_b_line).unwrap() > 0 {
            bot_a_writer.write_fmt(format_args!("{}", bot_b_line)).unwrap();
            bot_a_writer.flush().unwrap();
            stdout.write_fmt(format_args!("bot b: {}", bot_b_line)).unwrap();
            stdout.flush().unwrap();
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
    let bot_a_cmd = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let bot_b_cmd = if matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let bot_a_child = spawn_from_cmd(bot_a_cmd);
    let bot_b_child = spawn_from_cmd(bot_b_cmd);

    connect(bot_a_child, bot_b_child);
}
