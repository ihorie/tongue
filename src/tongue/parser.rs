use std::collections::HashMap;

use tongue::config::Config;

pub fn parse(input: &str, config: &Config) -> Vec<String> {
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
