// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;
extern crate tongue;

use std::{env};
use std::process::exit;
use std::collections::HashMap;

use tongue::{terminal, input};
use tongue::config::Config;

extern "C" {
  fn signal(sig: u32, cb: extern fn(u32));
}

extern fn interrupt(_:u32) {
}

const SIGINT: u32 = 2;

fn disable_sigint() {
    unsafe {
        signal(SIGINT, interrupt);
    }
}

fn tongue_main() {
    disable_sigint();
    let orig_term = terminal::tcgetattr();
    terminal::enable_raw_mode();
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
            terminal::tcsetattr(orig_term);
            exit(0);
        } else if argument == "--version" {
            print!("{}", env!("CARGO_PKG_VERSION"));
            terminal::tcsetattr(orig_term);
            exit(0);
        }
    }
    input::read_from_stdin(config, orig_term);
    terminal::tcsetattr(orig_term);
}

fn main() {
    tongue_main();
}
