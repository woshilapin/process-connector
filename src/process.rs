use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time;
use std::thread;

pub struct Process<'a> {
    pub name: &'a str,
    process: Child,
    tx: mpsc::Sender<Option<String>>,
    rx: mpsc::Receiver<Option<String>>,
    tx_err: mpsc::Sender<Option<String>>,
    rx_err: mpsc::Receiver<Option<String>>,
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
        let (tx_err, rx_err) = mpsc::channel();
        Process {
            name: name,
            process: process,
            tx: tx,
            rx: rx,
            tx_err: tx_err,
            rx_err: rx_err,
        }
    }

    pub fn init(&mut self, delay: u64) {
        let tx = self.tx.clone();
        let tx_err = self.tx_err.clone();
        let stdout = self.process.stdout.take().unwrap();
        let stderr = self.process.stderr.take().unwrap();

        thread::Builder::new().name(self.name.to_string()).spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                tx.send(Some(line.unwrap())).unwrap();
                thread::sleep(time::Duration::from_millis(delay));
            }
        }).unwrap();
        thread::Builder::new().name(self.name.to_string()).spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                tx_err.send(Some(line.unwrap())).unwrap();
                thread::sleep(time::Duration::from_millis(delay));
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

    pub fn pop_err(&mut self) -> Option<String> {
        let data = self.rx_err.try_recv();
        if data.is_ok() {
            data.unwrap()
        } else {
            None
        }
    }
}
