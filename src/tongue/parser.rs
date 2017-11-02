use tongue::node::Node;

pub fn parse (mut tokens: Vec<String>) -> Node {

    let mut root = Node {
        v: "".to_string(),
        child: Vec::new(),
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

fn insert(mut root: Node, v: &str) -> Node {
    let mut n = Node {
        v: v.to_string(),
        child: Vec::new(),
        sibling: None,
    };

    if v == "=" {
        n.child.push(root);
        root = n;
    } else {
        root.child.push(n);
    }
    root
}

#[test]
fn parse_empty() {
    let expected = Node {
        v: "".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    let got = parse(vec![]);
    assert_eq!(got, expected);
}


#[test]
fn parse_one_token() {
    {
        let expected = Node {
            v: "cd".to_string(),
            child: Vec::new(),
            sibling: None,
        };
        let got = parse(vec!["cd".to_string()]);
        assert_eq!(got, expected);
    }
}

#[test]
fn parse_two_token() {
    let mut expected = Node {
        v: "ls".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    let node = Node {
        v: "-l".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    expected.child.push(node);
    let got = parse(vec!["ls".to_string(), "-l".to_string()]);
    assert_eq!(got, expected);
}


#[test]
fn parse_three_token() {
    let mut expected = Node {
        v: "=".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    let node01 = Node {
        v: "ps1".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    let node02 = Node {
        v: "' $ '".to_string(),
        child: Vec::new(),
        sibling: None,
    };
    expected.child.push(node01);
    expected.child.push(node02);
    let got = parse(vec![
        "ps1".to_string(),
        "=".to_string(),
        "' $ '".to_string()
    ]);
    assert_eq!(got, expected);
}

