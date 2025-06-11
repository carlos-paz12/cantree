use crate::utils::pair::OrderedPair;

#[derive(Debug)]
pub struct Node<T> {
    pub key: u64,
    pub pair: OrderedPair,
    pub value: T,
    /* [!]
     * I think it's possible to use `*mut Node<T>` or `&mut Node<T>`...
     * But I don't know
     */
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}
