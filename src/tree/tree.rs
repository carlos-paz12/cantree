use super::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn with_root(other: Node) -> Self {
        Tree {
            /* [!]
             * The parameter `other` is moved into the Box, which is then stored in the `root` field.
             * After this line, `other` no longer exists (it has been consumed), because Rust uses
             * ownership and moves values by default.
             */
            root: Some(Box::new(other)),
        }
    }
}
