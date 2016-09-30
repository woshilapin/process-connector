use std::io::{BufRead, BufReader};

fn main() {
    loop {
        let mut line = String::new();
        let mut reader = BufReader::new(std::io::stdin());
        if reader.read_line(&mut line).unwrap() > 0 {
            if line.contains("I'm bot") {
                println!("I'm bot 'b' <{}>", line.trim());
            } else if line.contains("Nice") {
                println!("Nice to meet you too! <{}>", line.trim());
            }
        }
    }
}
