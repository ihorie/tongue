use tongue::node::Node;

pub fn parse (mut tokens: Vec<String>) -> Node {

    let mut root = Node {
        v: "".to_string(),
        l: None,
        r: None
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
        root.insert(token);
    }
    
    root
}

#[test]
fn parse_empty() {
    let expected = Node {
        v: "".to_string(),
        l: None,
        r: None
    };
    let got = parse(vec![]);
    assert_eq!(got, expected);
}

#[test]
fn parse_one_token() {
    {
        let expected = Node {
            v: "cd".to_string(),
            l: None,
            r: None
        };
        let got = parse(vec!["cd".to_string()]);
        assert_eq!(got, expected);
    }
}

#[test]
fn parse_two_token() {
    let mut expected = Node {
        v: "ls".to_string(),
        l: None,
        r: None,
    };
    let node = Some(Box::new(Node {
        v: "-l".to_string(),
        l: None,
        r: None,
    }));
    expected.r = node;
}

#[test]
fn parse_three_token() {
    let expected = Node {
        v: "".to_string(),
        l: None,
        r: None
    };
    
}
