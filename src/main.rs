//! ## main module
//!
//! This module runs basic and extended tests on the `BinarySearchTree`,
//! including insertion, search, and removal of various node configurations.

use cantree::{tree::bst::BinarySearchTree, utils::pair::OrderedPair};

fn main()
{
  let mut m_tree = BinarySearchTree::new();

  println!("Initial empty BST:\n{}\n", m_tree.to_string());

  // Inserção de nós
  m_tree.insert(OrderedPair::new(4, 4)); // key = 24
  m_tree.insert(OrderedPair::new(2, 1)); // key = 16
  m_tree.insert(OrderedPair::new(10, 10)); // key = 110
  m_tree.insert(OrderedPair::new(9, 9)); // key = 99
  m_tree.insert(OrderedPair::new(8, 8)); // key = 88
  m_tree.insert(OrderedPair::new(7, 7)); // key = 77
  m_tree.insert(OrderedPair::new(5, 5)); // key = 55

  println!("BST after inserting nodes:\n{}\n", m_tree.to_string());

  // Teste de busca
  let keys_to_search = [24, 110, 77, 999];
  for key in keys_to_search
  {
    match m_tree.search(key)
    {
      Some(pair) => println!(
        "Found key={} with pair=({}, {})",
        key, pair.first, pair.second
      ),
      None => println!("Key={} not found in BST", key),
    }
  }
  println!();

  // Teste de remoção de chave inexistente
  println!("Removing non-existing key=40...");
  m_tree.remove(40);
  println!(
    "BST after trying to remove key=40:\n{}\n",
    m_tree.to_string()
  );

  // Remover nó folha (sem filhos)
  println!("Removing leaf node with key=55...");
  m_tree.remove(55);
  println!("BST after removing leaf key=55:\n{}\n", m_tree.to_string());

  // Remover nó com um filho
  println!("Removing node with one child (key=88)...");
  m_tree.remove(88);
  println!("BST after removing node key=88:\n{}\n", m_tree.to_string());

  // Remover nó com dois filhos
  println!("Removing node with two children (key=99)...");
  m_tree.remove(99);
  println!("BST after removing node key=99:\n{}\n", m_tree.to_string());

  // Remover raiz
  println!("Removing root node (key=24)...");
  m_tree.remove(24);
  println!("BST after removing root key=24:\n{}\n", m_tree.to_string());

  // Mostrar BST final
  println!("Final BST state:\n{}", m_tree.to_string());
}
