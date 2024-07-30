use std::fs::read_to_string;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use felys::Worker;

use crate::cli::Cli;
use crate::rsffi::register;

mod cli;
mod rsffi;

struct Wrapper {
    worker: Worker,
    v: bool,
}

fn main() {
    let cli = Cli::parse();
    let mixin = register(&cli.lang);
    let worker = Worker::new(mixin, cli.timeout, cli.lang);

    let mut main = Wrapper { worker, v: cli.verbose };
    if let Some(file) = cli.file {
        main.execute_file(file)
    } else if main.start_interactive_interpreter().is_err() {
        println!("Your system is not supported for interactive Felys interpreter")
    }
}

impl Wrapper {
    fn execute_file(&mut self, file: PathBuf) {
        let code = read_to_string(file).unwrap();
        match self.worker.exec(code) {
            Ok(s) => if self.v {
                println!();
                println!("Finished with exit object <{}>", s.exit);
                println!("Time: {:?} | {:?} | {:?}", s.time.0, s.time.1, s.time.2)
            }
            Err(e) => { println!("{}", e) }
        }
    }

    fn start_interactive_interpreter(&mut self) -> io::Result<()> {
        println!("Felys {}", env!("CARGO_PKG_VERSION"));

        let mut history = Vec::new();
        let mut command = String::new();
        let mut index = 0;
        flush(format!("\r> {}", command));

        enable_raw_mode()?;
        loop {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Backspace, _) => {
                        command.pop();
                        flush(format!("\r> {} \r> {}", &command, &command))
                    }
                    (KeyCode::Char(c), _) => {
                        command.push(c);
                        flush(format!("\r> {}", command))
                    }
                    (KeyCode::Enter, _) => {
                         if !command.is_empty() {
                            println!("\r");
                            if let Err(e) = self.worker.exec(command.clone()) {
                                println!("{}", e)
                            }
                            history.push(command);
                            command = String::new();
                            flush(format!("\r> {}", command))
                        }
                        index = history.len();
                    }
                    (KeyCode::Up, _) => {
                        index = index.saturating_sub(1);
                        if let Some(nth) = history.get(index) {
                            flush(format!("\r{}", " ".repeat(command.len() + 2)));
                            command = nth.clone();
                            flush(format!("\r> {}", command))
                        }
                    }
                    (KeyCode::Down, _) => {
                        index = index.saturating_add(1);
                        if let Some(nth) = history.get(index) {
                            flush(format!("\r{}", " ".repeat(command.len() + 2)));
                            command = nth.clone();
                            flush(format!("\r> {}", command))
                        } else {
                            index = index.saturating_sub(1);
                        }
                    }
                    _ => ()
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}

fn flush(msg: String) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}