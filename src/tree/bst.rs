//! ## binary_search_tree module
//!
//! This module defines the `BinarySearchTree` struct and implements core
//! operations such as insertion, search, removal, and string representation
//! for debugging and visualization.

use super::node::Node;
use crate::utils::pair::OrderedPair;

/// Represents a binary search tree where each node contains an ordered pair.
/// Provides basic operations such as insertion, search, and removal.
#[derive(Debug)]
pub struct BinarySearchTree
{
  /// Root node of the binary search tree.
  pub root: Option<Box<Node>>,
}

impl BinarySearchTree
{
  /// ### Brief
  /// Creates a new, empty binary search tree.
  ///
  /// ### Returns
  /// A binary search tree with no nodes.
  pub fn new() -> Self
  {
    BinarySearchTree { root: None }
  }

  /// ### Brief
  /// Inserts a new ordered pair into the tree.
  ///
  /// ### Arguments
  /// * `pair` - The ordered pair to be inserted.
  ///
  /// ### Returns
  /// `true` if insertion was successful, `false` if a node with the same key
  /// already exists.
  pub fn insert(&mut self, pair: OrderedPair) -> bool
  {
    let new_node = Node::new(pair);
    Self::insert_node(&mut self.root, new_node)
  }

  /// ### Brief
  /// Searches for an ordered pair by its key.
  ///
  /// ### Arguments
  /// * `key` - The key to search for.
  ///
  /// ### Returns
  /// `Some(&OrderedPair)` if found, otherwise `None`.
  pub fn search(&self, key: u64) -> Option<&OrderedPair>
  {
    Self::search_node(&self.root, key)
  }

  /// ### Brief
  /// Removes a node from the tree by its key.
  ///
  /// ### Arguments
  /// * `key` - The key of the node to remove.
  ///
  /// ### Returns
  /// The removed node if it existed, otherwise `None`.
  pub fn remove(&mut self, key: u64) -> Option<Box<Node>>
  {
    Self::remove_node(&mut self.root, key)
  }

  /// ### Brief
  /// Recursive helper function for inserting a node into the tree.
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

  /// ### Brief
  /// Recursive helper function for searching a node by key.
  fn search_node(current: &Option<Box<Node>>, key: u64)
  -> Option<&OrderedPair>
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

  /// ### Brief
  /// Recursive helper function for removing a node by key.
  fn remove_node(root: &mut Option<Box<Node>>, key: u64) -> Option<Box<Node>>
  {
    match root
    {
      None =>
      {
        return None;
      }
      Some(node) =>
      {
        if key < node.key
        {
          return Self::remove_node(&mut node.left, key);
        }
        else if key > node.key
        {
          return Self::remove_node(&mut node.right, key);
        }
        else
        {
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
            (Some(left), Some(right)) =>
            {
              // Find the in-order successor (smallest in right subtree).
              let successor_key;
              let successor_pair;
              {
                let mut successor = right.as_ref();
                while let Some(ref left_child) = successor.left
                {
                  successor = left_child.as_ref();
                }
                successor_key = successor.key;
                successor_pair = successor.pair.clone();
              }

              node.key = successor_key;
              node.pair = successor_pair;
              node.left = Some(left);
              node.right = Some(right);

              // Remove the successor node from right subtree.
              Self::remove_node(&mut node.right, successor_key);

              return Some(node.clone());
            }
          }
        }
      }
    }
  }

  /// ### Brief
  /// Generates a string representation of the binary search tree for
  /// visualization.
  ///
  /// ### Returns
  /// A formatted string showing the structure of the tree.
  pub fn to_string(&self) -> String
  {
    fn helper(
      node: &Option<Box<Node>>,
      prefix: String,
      is_left: bool,
      is_root: bool,
    ) -> String
    {
      let mut output = String::new();

      output += &prefix;
      if is_root
      {
        output += ".\n";
      }

      output += if is_root
      {
        "└──"
      }
      else if is_left
      {
        "├E─"
      }
      else
      {
        "└D─"
      };

      match node
      {
        Some(n) =>
        {
          // Com cores (ANSI escape) para terminal, se quiser tirar cores,
          // remova \x1b[..m
          output += &format!(
            "\x1b[32m◉ [Key={}, Pair=({}, {})]\x1b[0m\n",
            n.key, n.pair.first, n.pair.second
          );

          let new_prefix = prefix
            + if is_root
            {
              "   "
            }
            else if is_left
            {
              "│  "
            }
            else
            {
              "   "
            };

          output += &helper(&n.left, new_prefix.clone(), true, false);
          output += &helper(&n.right, new_prefix.clone(), false, false);
        }
        None =>
        {
          output += "\x1b[31mx\x1b[0m\n";
        }
      }

      output
    }

    helper(&self.root, "".to_string(), false, true)
  }
}
