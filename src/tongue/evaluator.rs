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
    let mut command: String = "".to_string();
    let mut tokens: Vec<String> = Vec::new();
    
    loop {
        if current_node.options.is_empty() {
            match current_node.sibling {
                Some(n) => {
                    tokens.push(current_node.v.to_string());
                    current_node = *n;
                },
                None => {
                    command = current_node.v;
                    break;
                },
            }
        } else {
            match current_node.options.pop() {
                Some(n) => {
                    _eval(n, config);
                },
                None => {
                    break;
                },
            }
        }
    }

    if tokens.is_empty() {
        exec(&command, tokens, config);    
    } else if tokens.len() == 1 {
    }

}

fn exec(command: &str, options: Vec<String>, config: &mut Config) {
    match command {

        "alias" => {
            builtin::alias(options, config);
        },
        "cd" => {
            builtin::cd(options);
        },
        "ls" => {
            Command::new(command)
                .args(options)
                .status()
                .expect("command not found");
        },
        _ => {
        },
    }
}
