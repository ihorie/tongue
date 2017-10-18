extern crate tongue;

use std::io::{self, Write};
use std::io::BufReader;
use std::io::BufRead;
use std::process::exit;
use std::env;
use std::fs::File;

use tongue::parser;
use tongue::exec;

fn main() {
    tongue_main()
}

fn tongue_main() {
    
    for argument in env::args() {
        if argument == "--help" {
            println!("tongue [option]");
            exit(0);
        } else if argument == "--version" {
            println!("{}", env!("CARGO_PKG_VERSION"));
            exit(0);
        }
    }

    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(e)   => e.to_string(),
    };
    
    read_from_file(home + &"/.tonguerc");

    read_from_stdin();
}

fn read_from_file(path: String) {
    let file = File::open(path).expect("file not found");

    let reader = BufReader::new(file);

    for buf in reader.lines() {
        let v = parser::parse(&buf.expect("Failed to read file"));

        exec::exec(v);
        io::stdout().flush().unwrap();
    }
}

fn read_from_stdin() {
    loop {
        print!(" ¥ ");  
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)
            .expect("Failed to read line");
        if "".eq(&buf) {
            println!("");
            exit(0);
        }

        let v = parser::parse(&buf);

        exec::exec(v);
        io::stdout().flush().unwrap();
    }
}

