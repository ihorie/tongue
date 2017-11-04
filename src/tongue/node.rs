// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at you
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[derive(Clone)]
pub struct Node {
    pub v: String,
    pub options: Vec<Node>,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    pub sibling: Option<Box<Node>>,
}

//impl Node {
//}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        if self.v != other.v {
            return false;
        }
 
        if self.l != None && other.l != None {
            assert_eq!(self.l, other.l);
        } else if self.l == None && other.l != None {
            return false;
        } else if self.l != None && other.l == None {
            return false;
        }

        if self.r != None && other.l != None {
            assert_eq!(self.r, other.r);
        } else if self.r == None && other.r != None {
            return false;
        } else if self.r != None && other.r == None {
            return false;
        }
        
        if self.sibling != None && other.sibling != None {
            assert_eq!(self.sibling, other.sibling);
        } else if self.sibling != other.sibling {
            return false;
        }

        if self.options.len() != other.options.len() {
            return false;
        } else if self.options.len() > 0 {
            for i in 0..self.options.len() {
                assert_eq!(self.options[i], other.options[i]);
            }
        }

        true
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        s.push_str(&print_node(self.clone()));
        write!(f, "{}", s)
    }
}

pub fn print_node(node: Node) -> String {
    let mut s: String = String::from("");
    s.push_str("( ");

    s.push_str(&node.v);

    match node.l {
        Some(n) => {
            s.push_str(" ");
            s.push_str(&print_node(*n.clone()));
        },
        None => {
        },
    }

    match node.r {
        Some(n) => {
            s.push_str(" ");
            s.push_str(&print_node(*n.clone()));
        },
        None => {
        },
    }

    s.push_str(" )");
    
    s
}
