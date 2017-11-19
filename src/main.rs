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
use std::{mem, env, str};
use std::process::exit;
use std::fs::File;
use std::collections::HashMap;

use tongue::{lexer, parser, evaluator};
use tongue::config::Config;


/*** terminal settings ***/

extern "C" {
  fn signal(sig: u32, cb: extern fn(u32)) -> fn(u32);
}

extern fn interrupt(_:u32) {
}

fn disable_sigint() {
    let SIGINT = 2;
    unsafe {
        signal(SIGINT, interrupt);
    }
}

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

struct CursorPosition {
    x: usize,
    y: usize,
}

fn get_cursor_position() -> CursorPosition {
    let mut buf =  [0; 32];
    output("\x1b[6n");
    read_stdin(&mut buf);
    let buf = str::from_utf8(&buf[2 .. 32]).unwrap();
    let index_end = buf.find('R').unwrap();
    let index_middle = buf.find(';').unwrap();
    
    let cursor_position = CursorPosition {
        y: buf.get(0..index_middle)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        x: buf.get(index_middle+1..index_end)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    };

    cursor_position
}

/*** output ***/

fn output(s: &str) {
    let stdout = io::stdout();
    let mut buf_writer = BufWriter::new(stdout.lock());
    buf_writer.write(s.as_bytes());
    buf_writer.flush();
}

/*** input ***/

const CtrlB:     u8 = 2;
const CtrlC:     u8 = 3;
const CtrlD:     u8 = 4;
const CtrlF:     u8 = 6;
const Tab:       u8 = 9;
const Enter:     u8 = 13;
const CtrlU:     u8 = 21;
const Escape:    u8 = 27;
const Backspace: u8 = 127;

fn read_stdin(mut buffer: &mut [u8]) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read(&mut buffer);
}

fn read_from_stdin(config: &mut Config, orig_term: libc::termios) {
    let mut line = "".to_string();
    let ps1 = " $ ";
    output(ps1);
    loop {
        let mut buffer = [0; 4];
        read_stdin(&mut buffer);
        let c = buffer[0] as char;
        let stdout = io::stdout();
        let mut buf_writer = BufWriter::new(stdout.lock());
        match buffer[0] {
            CtrlB => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = get_cursor_position();
                if cursor_position.x == ps1.len() {
                    continue;
                }
                output("\x1b[D");
            }
            CtrlC => {
            }
            CtrlD => {
                if line.len() == 0 {
                    output("\r\n");
                    break;
                }
            }
            CtrlF => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = get_cursor_position();
                if cursor_position.x == ps1.len() {
                    continue;
                }
                output("\x1b[C");
            }
            Tab => {
            }
            Enter => {
                output("\r\n");
                tcsetattr(orig_term);
                let tokens = lexer::tokenize(line.as_str());
                let tree = parser::parse(tokens.clone());
                evaluator::eval(tree, config);
                enable_raw_mode();
                line = "".to_string();
                output(ps1);
            }
            CtrlU => {
                output("\x1b[0K");
                line = "".to_string();
            }
            Escape => {
                ;
            }
            Backspace => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = get_cursor_position();
                output(format!("\x1b[{};0H", cursor_position.y).as_str());
                output("\x1b[0K");
                output(ps1);
                line.remove(cursor_position.x-2-ps1.len());
                output(line.as_str());
                output(format!("\x1b[{};{}H", cursor_position.y, cursor_position.x-1).as_str());
            }
            _ => {
                let cursor_position = get_cursor_position();
                output(format!("\x1b[{};0H", cursor_position.y).as_str());
                output("\x1b[0K");
                output(ps1);
                line.insert(cursor_position.x-1-ps1.len(), c);
                output(line.as_str());
                output(format!("\x1b[{};{}H", cursor_position.y, cursor_position.x+1).as_str());
            }
        }
    }
}

fn read_rc(config: &mut Config) {
}

/*** main ***/

fn tongue_main() {
    disable_sigint();
    let orig_term = tcgetattr();
    enable_raw_mode();
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
            print!("tongue [option]\r\n");
            tcsetattr(orig_term);
            exit(0);
        } else if argument == "--version" {
            print!("{}", env!("CARGO_PKG_VERSION"));
            tcsetattr(orig_term);
            exit(0);
        }
    }
    read_rc(config);
    read_from_stdin(config, orig_term);
    tcsetattr(orig_term);
}

fn main() {
    tongue_main();
}

/*
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
