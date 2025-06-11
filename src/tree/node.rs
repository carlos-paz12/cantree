use crate::utils::pair::OrderedPair;

#[derive(Debug)]
pub struct Node {
    pub key: u64,
    pub pair: OrderedPair,
    /* [!]
     * I think it's possible to use `*mut Node` or `&mut Node`...
     * But I don't know
     */
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
