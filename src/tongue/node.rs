pub struct Node<'a> {
    pub v: &'a str,
    pub l: Option<Box<Node<'a>>>,
    pub r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
}
