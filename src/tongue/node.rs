use std::fmt;

pub struct Node {
    pub v: String,
    pub child: Vec<Node>,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, v: &str) {

        let n = Node {
            v: v.to_string(),
            child: Vec::new(),
            next: None,
        };

        self.next = Some(Box::new(n));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.v == other.v
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {}", self.v)
    }
}
