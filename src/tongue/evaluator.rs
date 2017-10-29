// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use tongue::builtin;
use tongue::config::Config;
use tongue::node::Node;

pub fn eval(tree: Node, mut config: &mut Config) {
    if tree.v.is_empty() {
        return;
    }

    
    if tree.v == "." {
        builtin::dot();
    } else if tree.v == "alias" {
    } else if tree.v == "break" {
    } else if tree.v == "cd" {
    } else if tree.v == "continue" {
    } else if tree.v == "exec" {
    } else if tree.v == "exit" {
    } else if tree.v == "export" {
    } else if tree.v == "return" {
    } else {
//        if options.is_empty() {
//            Command::new(command)
//                .status()
//                .expect("command failed to start");
//        } else {
//            Command::new(command)
//               .args(options)
//               .status()
//               .expect("command failed to start");
//        }
    }      
}

