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
    if token == "cd" {
        builtin::cd();
    } else {
        Command::new(token)
            .status()
            .expect("command failed to start");
    }
}
