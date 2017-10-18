extern crate tongue;

use std::io::{self, Write};
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

    read_from_file("~/.tonguerc");

    read_from_stdin();
}

fn read_from_file(path: &str) {
    let mut file = File::open(path);
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

