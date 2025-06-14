use cantree::{tree::bst::BinarySearchTree, utils::pair::OrderedPair};

fn main() {
    let mut m_tree = BinarySearchTree::new();
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(1, 2));
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(0, 0));
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(3, 9));
    println!("{:#?}", m_tree);

    if let Some(p) = m_tree.search(87) {
        println!("{:#?}", p); // [!] Espera como saída o último par adicionado na árvore.
    }
}
