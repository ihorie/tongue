// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use builtin;

pub fn exec(tokens: Vec<String>) {
    if tokens[0] == "." {
        builtin::dot();
    } else if tokens[0] == "break" {
        builtin::_break();
    } else if tokens[0] == "cd" {
        builtin::cd(tokens);
    } else if tokens[0] == "continue" {
        builtin::_continue();
    } else if tokens[0] == "exec" {
        builtin::exec();
    } else if tokens[0] == "exit" {
        builtin::exit();
    } else if tokens[0] == "export" {
        builtin::export();
    } else if tokens[0] == "return" {
        builtin::_return();
    } else {
        if let Some((command, options)) = tokens.split_first() {
            if options.is_empty() {
                Command::new(command)
                    .status()
                    .expect("command failed to start");
            } else {
                Command::new(command)
                    .args(options)
                    .status()
                    .expect("command failed to start");
            }
        }
    }
}
