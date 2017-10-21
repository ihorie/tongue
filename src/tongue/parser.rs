use std::collections::HashMap;

use tongue::config::Config;

pub fn parse(buf: &str, config: &Config) -> Vec<String> {
    let mut token: String = String::from("");

    let mut v: Vec<String> = Vec::new();
    
    for c in buf.chars() {
        if c == ' ' {
            println!("{}", token);
            v.push(token.clone());
            token = String::from("");
//        } else if c == '"' {
//            continue;
        } else if c == '=' {
            v.push(token.clone());
            token = String::from("");
        } else if c == '\n' {
            v.push(token.clone());
            token = String::from("");
        } else {
            token.push(c);
            //token = String::from("");
        }
    }
    
    v
}
