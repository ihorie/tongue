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

fn _eval(mut n: Node, config: &mut Config) {
    if n.v == "=" {
//        config.variable.insert("ps1", " # ");
        match n.l {
            Some(l) => {
                match n.r {
                    Some(r) => {
                        config.variable.insert(l.v, r.v);
                    },
                    None => {
                        // should be error!!
                    },
                }
            },
            None => {
                // should be error!!
            },
        }
    } else if n.v == "alias" {
        exec(n, config);
    } else if n.v == "cd" {
        exec(n, config);
    } else {
        exec(n, config);
    }
}

fn exec(n: Node, config: &mut Config) {
    if n.v == "alias" {
        builtin::alias(n.options, config);
    } else if n.v == "cd" {
        builtin::cd(n.options);
    } else {
        match Command::new(n.v)
            .args(n.options)
            .status()
        {
            Ok(n) => {},
            Err(err) => {
                println!("{}", err);
            },
        }

    }
}
