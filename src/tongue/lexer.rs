use tongue::config::Config;

pub fn tokenize(input: &str, config: &Config) -> Vec<String> {
    debug!("{}", input);
    
    let mut token: String = String::from("");
    
    let mut v: Vec<String> = Vec::new();

    let mut chars = input.chars();

    let mut is_string = false;
    
    loop {
        if is_string == true {
            match chars.next() {
                Some(c)  => {
                    match c {
                        '"' => {
                            if token.is_empty() == false {
                                v.push(token);
                            }
                            token = String::from("");
                            is_string = false;
                        },
                        _ => {
                            token.push(c);
                        }
                    };
                },
                None => {
                    break;
                },
            };
            continue;
        }
        match chars.next() {            
            Some(c) => {
                match c {
                    ' ' => {
                        if token.is_empty() == false {
                            v.push(token);
                        }
                        token = String::from("");
                    },
                    '=' => {
                        if token.is_empty() == false {
                            v.push(token);
                        }
                        token = String::from("=");
                        v.push(token);
                        token = String::from("");
                    },
                    '"' => {
                        if token.is_empty() == false {
                            v.push(token);
                        }
                        token = String::from("");
                        is_string = true;
                    },
                    '\n' => {
                        if token.is_empty() == false {
                            v.push(token);
                        }
                        token = String::from("");
                        break;
                    },
                    _ => token.push(c),
                }
            },
            None => {
                if token.is_empty() == false {
                    v.push(token);
                }
                break;
            },
        }
    }
    
    v
}


#[test]
fn tokenize_empty() {
    use std::collections::HashMap;

    let config = &mut Config {
        aliases: HashMap::new(),
        home : "HOME".to_string(),
    };
    let expected: Vec<String> = Vec::new();
    let got = tokenize("", &config);
    assert_eq!(got, expected);
}

#[test]
fn tokenize_one_token() {
    use std::collections::HashMap;

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec!["cd".to_string()];
        let got = tokenize("cd", &config);
        assert_eq!(got, expected);
    }

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec!["ls".to_string()];
        let got = tokenize("ls\n", &config);
        assert_eq!(got, expected);
    }
}

#[test]
fn tokenize_two_token() {
    use std::collections::HashMap;

    let config = &mut Config {
        aliases: HashMap::new(),
        home : "HOME".to_string(),
    };
    let expected: Vec<String> = vec!["cd".to_string(), "directory".to_string()];
    let got = tokenize("cd directory", &config);
    assert_eq!(got, expected);
}

#[test]
fn tokenize_three_token() {
    use std::collections::HashMap;

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec![
            "ls".to_string(),
            "-l".to_string(),
            "directory".to_string(),
        ];
        let got = tokenize("ls -l directory", &config);
        assert_eq!(got, expected);
    }

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home: "HOME".to_string(),
        };
        let expected: Vec<String> = vec![
            "PS1".to_string(),
            "=".to_string(),
            ">".to_string(),
        ];
        let got = tokenize("PS1 = \">\"", &config);
        assert_eq!(got, expected);
    }
}

#[test]
fn tokenize_four_token() {
    use std::collections::HashMap;

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec![
            "alias".to_string(),
            "emacs".to_string(),
            "=".to_string(),
            "emacs -nw".to_string()
        ];
        let got = tokenize("alias emacs = \"emacs -nw\"", &config);
        assert_eq!(got, expected);
    }
}
