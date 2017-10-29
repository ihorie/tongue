extern crate tongue;

use std::io::{self, Write, BufReader, BufRead};
use std::process::exit;
use std::env;
use std::fs::File;
use std::collections::HashMap;

use tongue::lexer;
use tongue::parser;
use tongue::exec;
use tongue::config::Config;

fn main() {
    tongue_main();
}

fn tongue_main() {
    let mut config = &mut Config {
        aliases: HashMap::new(),
        home : match env::var("HOME") {
            Ok(home) => home,
            Err(e) => e.to_string(),
        },
    };

    for argument in env::args() {
        if argument == "--help" {
            println!("tongue [option]");
            exit(0);
        } else if argument == "--version" {
            println!("{}", env!("CARGO_PKG_VERSION"));
            exit(0);
        }
    }

    read_rc(config);

    read_from_stdin(config);
}


fn read_rc(config: &mut Config) {
    read_from_file("/.tonguerc".to_string(), config);
}

fn read_from_file(path: String, config: &mut Config) {
    
    let file = File::open(config.home.clone() + &path).expect("file not found");

    let reader = BufReader::new(file);

    for buf in reader.lines() {
        let tokens = lexer::tokenize(&buf.expect("Failed to read file"), &config);

        //parser::parse(tokens.clone());
        
        exec::exec(tokens, config);
        
        io::stdout().flush().unwrap();
    }

}

fn read_from_stdin(config: &mut Config) {
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

        let tokens = lexer::tokenize(&buf, &config);

        //parser::parse(tokens.clone());
        
        exec::exec(tokens, config);

        io::stdout().flush().unwrap();
    }
}
