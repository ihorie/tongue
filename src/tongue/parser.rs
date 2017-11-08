// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at you
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use tongue::node::Node;

pub fn parse (mut tokens: Vec<String>) -> Node {

    let mut root = Node {
        v: "".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };

    match tokens.first() {
        Some(token) => {
            root.v = token.to_string();
        },
        None => {
            return root;
        }
    }

    tokens.remove(0);

    for token in &tokens {
        root = insert(root, token);
    }

    root
}

fn insert (mut root: Node, v: &str) -> Node {
    let mut n = Node {
        v: v.to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };

    match v {
        "=" => {
            n.l = Some(Box::new(root));
            root = n;
        },
        "+" => {
            if root.v == "=" {
                let tmp = root.clone();
                match tmp.r {
                    Some(r) => {
                        root.r = Some(Box::new(insert(*r, v)));
                    },
                    None => {
                        // should be ERROR!
                    },
                }
            } else {
                n.l = Some(Box::new(root));
                root = n;
            }
        },
        _ => {
            if root.v == "=" {
                let tmp = root.clone();
                match tmp.r {
                    Some(r) => {
                        root.r = Some(Box::new(insert(*r, v)));
                    },
                    None => {
                        root.r = Some(Box::new(n));
                    },
                }
            } else if root.v == "+" {
                if root.r == None {                    
                    root.r = Some(Box::new(n));
                }
            } else {
                root.options.push(v.to_string());
            }
        }
    }
    root
}

#[test]
fn parse_empty() {
    let expected = Node {
        v: "".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let got = parse(vec![]);
    assert_eq!(got, expected);
}

#[test]
fn parse_command_without_args() {
    {
        let expected = Node {
            v: "cd".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        let got = parse(vec!["cd".to_string()]);
        assert_eq!(got, expected);
    }
}

#[test]
fn parse_command_with_args() {
    let mut expected = Node {
        v: "ls".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let node = Node {
        v: "-l".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    expected.options.push("-l".to_string());
    let got = parse(vec!["ls".to_string(), "-l".to_string()]);
    assert_eq!(got, expected);
}

#[test]
fn parse_variable_binding() {
    let mut expected = Node {
        v: "=".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let node01 = Node {
        v: "ps1".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let node02 = Node {
        v: "' $ '".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    expected.l = Some(Box::new(node01));
    expected.r = Some(Box::new(node02));
    let got = parse(vec![
        "ps1".to_string(),
        "=".to_string(),
        "' $ '".to_string()
    ]);
    assert_eq!(got, expected);
}

#[test]
fn parse_plus() {
    let mut expected = Node {
        v: "+".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let node01 = Node {
        v: "1".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    let node02 = Node {
        v: "2".to_string(),
        options: Vec::new(),
        l: None,
        r: None,
        sibling: None,
    };
    expected.l = Some(Box::new(node01));
    expected.r = Some(Box::new(node02));
    let got = parse(vec![
        "1".to_string(),
        "+".to_string(),
        "2".to_string()
    ]);
    assert_eq!(got, expected);
}

#[test]
fn parse_formula() {
    {
        let mut expected = Node {
            v: "=".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        let node01 = Node {
            v: "x".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        let mut node02 = Node {
            v: "+".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        let node03 = Node {
            v: "1".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        let node04 = Node {
            v: "2".to_string(),
            options: Vec::new(),
            l: None,
            r: None,
            sibling: None,
        };
        node02.l = Some(Box::new(node03));
        node02.r = Some(Box::new(node04));
        expected.l = Some(Box::new(node01));
        expected.r = Some(Box::new(node02));
        let got = parse(vec![
            "x".to_string(),
            "=".to_string(),
            "1".to_string(),
            "+".to_string(),
            "2".to_string()
        ]);
        assert_eq!(got, expected);
    }
}
