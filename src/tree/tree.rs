use super::node::Node;
use crate::utils::pair::OrderedPair;

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

    pub fn insert(&mut self, pair: OrderedPair) -> bool {
        let new_node = Node::new(pair);
        Self::insert_node(&mut self.root, new_node)
    }

    fn insert_node(current: &mut Option<Box<Node>>, new_node: Node) -> bool {
        match current {
            None => {
                *current = Some(Box::new(new_node));
                return true;
            }
            Some(existing_node) => {
                if new_node.key < existing_node.key {
                    Self::insert_node(&mut existing_node.left, new_node)
                } else if new_node.key > existing_node.key {
                    Self::insert_node(&mut existing_node.right, new_node)
                } else {
                    return false;
                }
            }
        }
    }
}
