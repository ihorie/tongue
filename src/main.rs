// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate tongue;

use std::io::{self, Write, BufReader, BufRead};
use std::process::exit;
use std::env;
use std::fs::File;
use std::collections::HashMap;

use tongue::lexer;
use tongue::parser;
use tongue::evaluator;
use tongue::config::Config;

fn main() {
    tongue_main();
}

fn tongue_main() {
    let config = &mut Config {
        aliases: HashMap::new(),
        home: String::new(),
    };

    match env::var("HOME") {
        Ok(home) => config.home = home,
        Err(e) => config.home = e.to_string(),
    }

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
        let tokens = lexer::tokenize(&buf.expect("Failed to read file"));
        let tree = parser::parse(tokens.clone());
        evaluator::eval(tree, config);
        io::stdout().flush().unwrap();
    }
}

fn read_from_stdin(config: &mut Config) {
    loop {
        let ps1: String;
        match env::var("PS1") {
            Ok(val) => ps1 = val,
            Err(e) => {
                //println!("{}", e.to_string());
                ps1 = " $ ".to_string();
            }
        }
        print!("{}", ps1);
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        if "".eq(&buf) {
            println!("");
            exit(0);
        }
        let tokens = lexer::tokenize(&buf);
        let tree =  parser::parse(tokens.clone());
        evaluator::eval(tree, config);
        io::stdout().flush().unwrap();
    }
}
