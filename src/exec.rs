// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use builtin;

pub fn exec(token: &str) {
    if token == "break" {
        builtin::_break();
    } else if token == "cd" {
        builtin::cd();
    } else if token == "continue" {
        builtin::_continue();
    } else if token == "exec" {
        builtin::exec();
    } else if token == "exit" {
        builtin::exit();
    } else if token == "export" {
        builtin::export();
    } else {
        Command::new(token)
            .status()
            .expect("command failed to start");
    }
}
