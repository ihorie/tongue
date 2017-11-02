use std::fmt;

pub struct Node {
    pub v: String,
    pub child: Vec<Node>,
    pub sibling: Option<Box<Node>>,
}

//impl Node {
//}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        if self.v != other.v {
            return false;
        }
        
        if self.sibling != None && other.sibling != None {
            assert_eq!(self.sibling, other.sibling);
        } else if self.sibling != other.sibling {
            return false;
        }

        if self.child.len() != other.child.len() {
            return false;
        } else if self.child.len() > 0 {
            for i in 0..self.child.len() {
                assert_eq!(self.child[i], other.child[i]);
            }
        }

        
        true
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        s.push_str("Value: ");
        s.push_str(&self.v);
        s.push_str(", ");
        s.push_str("Childs: ");
        let l: String = self.child.len().to_string();
        s.push_str(&l);

        write!(f, "{}", s)
    }
}
