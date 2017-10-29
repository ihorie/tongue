use tongue::node::Node;

pub fn parse(tokens: Vec<String>) {

    let mut root = Node {
        v: "root",
        l: None,
        r: None
    };
    
    for token in &tokens {
        let mut node = Node {
            v: token,
            l: None,
            r: None
        };
    };
}
