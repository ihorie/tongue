// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;
extern crate tongue;

use std::io::{self, Write, BufWriter, Read, BufReader, BufRead};
use std::{mem, env};
use std::process::exit;
use std::fs::File;
use std::collections::HashMap;

use tongue::{lexer, parser, evaluator};
use tongue::config::Config;

fn tcgetattr() -> libc::termios {
    let mut term: libc::termios;
    unsafe {
        term = mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut term);
    }
    term
}

fn tcsetattr(mut term: libc::termios) {
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut term);
    }
}

fn enable_raw_mode() {
    unsafe {
        let mut term: libc::termios = mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut term);
        term.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        term.c_oflag &= !(libc::OPOST);
        term.c_cflag |= (libc::CS8);
        term.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        //term.c_cc[libc::VMIN] = 0;
        //term.c_cc[libc::VTIME] = 1;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut term);
    }
}

fn main() {
    tongue_main();
}

fn tongue_main() {
    let mut orig_term = tcgetattr();
    enable_raw_mode();
    loop {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = [0; 1];
        handle.read(&mut buffer);
        let c = buffer[0] as char;
        match buffer[0] {
            4 => {
                tcsetattr(orig_term);
                exit(1);
            },
            _ => {
                let stdout = io::stdout();
                let mut buf = BufWriter::new(stdout.lock());
                buf.write(&buffer);
                buf.flush();
            }
        }
    }
}

/*
fn tongue_main() {
    let config = &mut Config {
        aliase: HashMap::new(),
        variable: HashMap::new(),
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
        prompt(config);
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

fn prompt(config: &mut Config) {
    match config.variable.get("ps1") {
        Some(ps1) => print!("{}", ps1),
        None => print!(" $ "),
    }
}
*/
