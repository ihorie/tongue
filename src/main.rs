extern crate tongue;

use std::io::{self, Write};
use std::process::exit;
use std::env;

use tongue::parser;
use tongue::exec;

fn main() {
    tongue_main()
}

fn tongue_main() {
    
    for argument in env::args() {
        if argument == "--help" {
            println!("tongue [option]");
        }
    }
    
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

