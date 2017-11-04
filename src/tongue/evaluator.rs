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

pub fn eval(tree: Node, config: &mut Config) {
    if tree.v.is_empty() {
        return;
    }

    _eval(tree, config);
}

fn _eval(mut current_node: Node, config: &mut Config) {
    if current_node.v == "alias" {
    } else if current_node.v == "cd" {
        exec(current_node, config);
    } else if current_node.v == "ls" {
        exec(current_node, config);
    }    
}

fn exec(n: Node, config: &mut Config) {
    if n.v == "alias" {
        builtin::alias(n.options, config);
    } else if n.v == "cd" {
        builtin::cd(n.options);
    } else if n.v == "ls" {
        Command::new(n.v)
            .args(n.options)
            .status()
            .expect("command not found");
    }
}
