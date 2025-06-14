use super::node::Node;
use crate::utils::pair::OrderedPair;

#[derive(Debug)]
pub struct BinarySearchTree
{
    pub root: Option<Box<Node>>,
}

impl BinarySearchTree
{
    pub fn new() -> Self
    {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, pair: OrderedPair) -> bool
    {
        let new_node = Node::new(pair);
        Self::insert_node(&mut self.root, new_node)
    }

    pub fn search(&self, key: u64) -> Option<&OrderedPair>
    {
        Self::search_node(&self.root, key)
    }

    pub fn remove(&mut self, key: &u64) -> Option<Box<Node>>
    {
        Self::remove_node(&mut self.root, key)
    }

    fn insert_node(current: &mut Option<Box<Node>>, new_node: Node) -> bool
    {
        match current
        {
            None =>
            {
                *current = Some(Box::new(new_node));
                return true;
            }
            Some(existing_node) =>
            {
                if new_node.key < existing_node.key
                {
                    Self::insert_node(&mut existing_node.left, new_node)
                }
                else if new_node.key > existing_node.key
                {
                    Self::insert_node(&mut existing_node.right, new_node)
                }
                else
                {
                    return false;
                }
            }
        }
    }

    fn search_node(current: &Option<Box<Node>>, key: u64) -> Option<&OrderedPair>
    {
        match current
        {
            Some(existing_node) =>
            {
                if key == existing_node.key
                {
                    Some(&existing_node.pair)
                }
                else if key < existing_node.key
                {
                    Self::search_node(&existing_node.left, key)
                }
                else
                {
                    Self::search_node(&existing_node.right, key)
                }
            }
            None => None,
        }
    }

    fn remove_node(root: &mut Option<Box<Node>>, key: &u64) -> Option<Box<Node>>
    {
        match root
        {
            None => None,
            Some(node) =>
            {
                if key < &node.key
                {
                    return Self::remove_node(&mut node.left, key);
                }
                else if key > &node.key
                {
                    return Self::remove_node(&mut node.right, key);
                }

                match (node.left.take(), node.right.take())
                {
                    (None, None) =>
                    {
                        return root.take();
                    }
                    (Some(child), None) | (None, Some(child)) =>
                    {
                        let node_to_remove = root.take().unwrap();
                        *root = Some(child);
                        return Some(node_to_remove);
                    }
                    (Some(_left), Some(_right)) =>
                    {
                        return None;
                    }
                }
            }
        }
    }
}
