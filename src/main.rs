use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use std::io::Write;
use clap::Parser;
use felys::{Language, Object, Worker};
use crate::cli::Cli;

mod cli;
mod rsffi;

fn main() {
    let cli = Cli::parse();
    let mixin = match cli.lang {
        Language::CN => HashMap::from([
            ("打印".into(), Object::Rust { func: rsffi::print }),
            ("输入".into(), Object::Rust { func: rsffi::input })
        ]),
        Language::EN => HashMap::from([
            ("print".into(), Object::Rust { func: rsffi::print }),
            ("input".into(), Object::Rust { func: rsffi::input })
        ])
    };
    
    let mut main = Worker::new(mixin, 0.0, cli.lang);
    if let Some(file) = cli.file {
        let code = read_to_string(file).expect("no such file");
        match main.exec(code) {
            Ok(o) => if cli.verbose {
                println!("\nFinished with <{}> in {:?}", o.code, o.duration)
            },
            Err(e) => println!("{}", e)
        }
    } else {
        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();

            let mut code = String::new();
            if io::stdin().read_line(&mut code).is_err() {
                println!("failed to read input");
            } else if code.as_str() == "exit\n" {
                break;
            }

            if let Err(e) = main.exec(code) {
                println!("{}", e)
            }
        }
    }
}
