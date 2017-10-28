use tongue::config::Config;

pub fn parse(input: &str, config: &Config) -> Vec<String> {
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
fn parse_empty() {
    use std::collections::HashMap;

    let config = &mut Config {
        aliases: HashMap::new(),
        home : "HOME".to_string(),
    };
    let expected: Vec<String> = Vec::new();
    let got = parse("", &config);
    assert_eq!(got, expected);
}

#[test]
fn parse_one_token() {
    use std::collections::HashMap;

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec!["cd".to_string()];
        let got = parse("cd", &config);
        assert_eq!(got, expected);
    }

    {
        let config = &mut Config {
            aliases: HashMap::new(),
            home : "HOME".to_string(),
        };
        let expected: Vec<String> = vec!["ls".to_string()];
        let got = parse("ls\n", &config);
        assert_eq!(got, expected);
    }
}

#[test]
fn parse_two_token() {
    use std::collections::HashMap;

    let config = &mut Config {
        aliases: HashMap::new(),
        home : "HOME".to_string(),
    };
    let expected: Vec<String> = vec!["cd".to_string(), "directory".to_string()];
    let got = parse("cd directory", &config);
    assert_eq!(got, expected);
}

#[test]
fn parse_three_token() {
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
        let got = parse("ls -l directory", &config);
        assert_eq!(got, expected);
    }

}

#[test]
fn parse_four_token() {
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
        let got = parse("alias emacs = \"emacs -nw\"", &config);
        assert_eq!(got, expected);
    }
}
