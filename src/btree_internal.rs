use std::fmt::Debug;

type ChildNode<T> = Option<Box<BtNode<T>>>;
#[derive(Debug)]
pub struct BtNode<T> {
    pub left: ChildNode<T>,
    pub right: ChildNode<T>,
    pub op: T,
}
