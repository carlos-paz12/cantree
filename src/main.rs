use cantree::tree::tree::Tree;
use cantree::utils::pair::OrderedPair;

fn main() {
    let mut m_tree = Tree::new();
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(1, 2));
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(0, 0));
    println!("{:#?}\n", m_tree);

    m_tree.insert(OrderedPair::new(3, 9));
    println!("{:#?}", m_tree);
}
