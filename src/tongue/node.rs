use std::fmt;

pub struct Node {
    pub v: String,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, v: &str) {

        match self.r {
            Some(ref mut n) => {
                n.insert(v);
            },
            None => {
                self.r = Some(Box::new(Node {
                    v: v.to_string(),
                    l: None,
                    r: None
                }));
            }
        }
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
