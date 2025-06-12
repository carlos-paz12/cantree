use crate::utils::cantor::gen_key;
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

impl Node {
    pub fn new(_pair_: OrderedPair) -> Self {
        let _key_ = gen_key(_pair_.first, _pair_.second);
        Node {
            key: _key_,
            pair: _pair_,
            left: None,
            right: None,
        }
    }
}
