//! ## node module
//!
//! This module defines the `Node` struct.
//! Each node stores an OrderedPair, a unique key generated using the Cantor
//! pairing function, and optional left and right child nodes.
use crate::utils::cantor::gen_key;
use crate::utils::pair::OrderedPair;

/// Represents a node in a binary tree that holds an ordered pair and a unique
/// key.
#[derive(Debug, Clone)]
pub struct Node
{
  /// Key used to guarantee the uniqueness of the node.
  pub key: u64,
  /// The ordered pair stored in the node.
  pub pair: OrderedPair,
  /// Left child of the node in the binary tree.
  pub left: Option<Box<Node>>,
  /// Right child of the node in the binary tree.
  pub right: Option<Box<Node>>,
}

impl Node
{
  /// ### Brief
  /// Creates a new Node with the given ordered pair. The key is generated
  /// using the Cantor pairing function to ensure uniqueness.
  ///
  /// ### Arguments
  /// * `ordered_pair` - The ordered pair to be stored in the node.
  ///
  /// ### Returns
  /// A new Node with a unique key and no children.
  pub fn new(ordered_pair: OrderedPair) -> Self
  {
    let key = gen_key(&ordered_pair);
    return Node {
      key: key,
      pair: ordered_pair,
      left: None,
      right: None,
    };
  }
}
