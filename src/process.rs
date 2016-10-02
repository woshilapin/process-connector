use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;

pub struct Process<'a> {
    pub name: &'a str,
    process: Child,
    tx: mpsc::Sender<Option<String>>,
    rx: mpsc::Receiver<Option<String>>,
}

impl<'a> Process<'a> {
    pub fn new(name: &str, cmd: String) -> Process {
        let cmd_array: Vec<&str> = cmd.split(" ").collect();
        let process =
            Command::new(cmd_array[0])
            .args(&cmd_array[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        let (tx, rx) = mpsc::channel();
        Process {
            name: name,
            process: process,
            tx: tx,
            rx: rx,
        }
    }

    pub fn init(&mut self) {
        let tx = self.tx.clone();
        let stdout = self.process.stdout.take().unwrap();

        thread::Builder::new().name(self.name.to_string()).spawn(move || {
            let current = thread::current();
            let reader = BufReader::new(stdout);
            let name = match current.name() {
                Some(n) => { n }
                None => { "default" }
            };

            for line in reader.lines() {
                tx.send(Some(line.unwrap())).unwrap();
                use std::time;
                thread::sleep(time::Duration::from_millis(500));
            }
        }).unwrap();
    }

    pub fn push(&mut self, msg: String) {
        let stdin = self.process.stdin.as_mut().unwrap();
        let mut writer = BufWriter::new(stdin);

        writer.write_fmt(format_args!("{}\n", msg)).unwrap();
        writer.flush().unwrap();
    }

    pub fn pop(&mut self) -> Option<String> {
        let data = self.rx.try_recv();
        if data.is_ok() {
            data.unwrap()
        } else {
            None
        }
    }
}
