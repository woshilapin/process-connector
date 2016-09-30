use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello world!");
    println!("I'm bot 'a'");
    let mut line = String::new();
    let mut reader = BufReader::new(std::io::stdin());
    if reader.read_line(&mut line).unwrap() > 0 {
        println!("Nice to meet you. <{}>", line.trim());
    }
}
