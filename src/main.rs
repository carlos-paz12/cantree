use cantree::{tree::bst::BinarySearchTree, utils::pair::OrderedPair};

fn main()
{
    let mut m_tree = BinarySearchTree::new();
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(0, 0));
    m_tree.insert(OrderedPair::new(1, 2));
    m_tree.insert(OrderedPair::new(0, 1));
    m_tree.insert(OrderedPair::new(3, 9));

    println!("{:#?}\n", m_tree);

    if let Some(node) = m_tree.remove(&87)
    {
        println!("{:#?}", node);
    }

    println!("{:#?}\n", m_tree);
    if let Some(node) = m_tree.remove(&8)
    {
        println!("{:#?}", node);
    }

    println!("{:#?}\n", m_tree);
}
