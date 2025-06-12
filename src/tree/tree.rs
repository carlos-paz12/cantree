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

    pub fn search(&self, key: u64) -> Option<&OrderedPair> {
        Self::search_node(&self.root, key)
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

    fn search_node(current: &Option<Box<Node>>, key: u64) -> Option<&OrderedPair> {
        match current {
            Some(existing_node) => {
                if key == existing_node.key {
                    Some(&existing_node.pair)
                } else if key < existing_node.key {
                    Self::search_node(&existing_node.left, key)
                } else {
                    Self::search_node(&existing_node.right, key)
                }
            }
            None => None
        }
    }

}
