use std::collections::HashMap;

use tongue::config::Config;

pub fn parse_old(input: &str, config: &Config) -> Vec<String> {
    let mut token: String = String::from("");

    let mut v: Vec<String> = Vec::new();

    let mut is_str = false;
    
    for c in input.chars() {
        if c == ' ' {
            if is_str {
                token.push(c);
            } else {
                 v.push(token.clone());
                token = String::from("");
            }
        } else if c == '"' {
            if is_str {
                is_str = false;
            } else {
                is_str = true;
            }
        } else if c == '=' {
            token.push(c);
        } else if c == '\n' {
            v.push(token.clone());
            token = String::from("");
        } else {
            token.push(c);
        }
    }

    if token.is_empty() {
    } else {
        v.push(token.clone());
    }

    v
}

pub fn parse(input: &str) -> Vec<String> {
    let mut token: String = String::from("");
    
    let mut v: Vec<String> = Vec::new();

    let mut chars = input.chars();

    loop {
        match chars.next() {
            Some(c) => {token.push(c)},
            None => {
                if token.is_empty() == false {
                    v.push(token);
                }
                break;
            }
            _ => break,
        }
    }
    
    v
}

#[test]
fn parse_empty() {
    let expected: Vec<String> = Vec::new();
    let got = parse("");
    assert_eq!(got, expected);
}

#[test]
fn parse_one_token() {
    let expected: Vec<String> = vec!["cd".to_string()];
    let got = parse("cd");
    assert_eq!(got, expected);
}

#[test]
fn parse_two_token() {
    let expected: Vec<String> = vec!["cd".to_string(), "directory".to_string()];
    let got = parse("cd directory");
}
