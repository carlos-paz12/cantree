use cantree::{tree::bst::BinarySearchTree, utils::pair::OrderedPair};

fn main()
{
  let mut m_tree = BinarySearchTree::new();
  println!("{}", m_tree.to_string());

  m_tree.insert(OrderedPair::new(4, 4));
  m_tree.insert(OrderedPair::new(2, 1));
  m_tree.insert(OrderedPair::new(10, 10));
  m_tree.insert(OrderedPair::new(9, 9));
  m_tree.insert(OrderedPair::new(8, 8));
  m_tree.insert(OrderedPair::new(7, 7));
  m_tree.insert(OrderedPair::new(5, 5));

  // println!("{:#?}\n", m_tree);

  // if let Some(node) = m_tree.remove(&87) {
  //     println!("{:#?}", node);
  // }

  println!("BST after insert Nodes:\n{}\n", m_tree.to_string());

  m_tree.remove(40);

  println!(
    "BST after remove node with key=40:\n{}\n",
    m_tree.to_string()
  );
}
