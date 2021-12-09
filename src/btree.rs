use crate::operators::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
pub enum NodeType<T> {
    Operation(Box<dyn Op<T>>),
    Val(Box<dyn Operand<T>>),
}

pub trait ExpressionExecute<T> {
    fn execute(self) -> T;
}

type ChildNode<T> = Option<Box<BTNode<T>>>;
#[derive(Debug)]
pub struct BTNode<T> {
    pub left: ChildNode<T>,
    pub right: ChildNode<T>,
    pub op: NodeType<T>,
}

impl<T> BTNode<T> {
    pub fn new(val: impl Operand<T> + 'static) -> Self {
        BTNode {
            left: None,
            right: None,
            op: NodeType::Val(Box::new(val)),
        }
    }

    pub fn op(operator: impl Op<T> + 'static, lhs: BTNode<T>, rhs: BTNode<T>) -> Self {
        BTNode {
            left: Some(Box::new(lhs)),
            right: Some(Box::new(rhs)),
            op: NodeType::Operation(Box::new(operator)),
        }
    }
}

impl<T: Add<Output = T>> Add for BTNode<T> {
    type Output = BTNode<T>;

    fn add(self, rhs: Self) -> Self::Output {
        BTNode {
            left: Some(Box::new(self)),
            right: Some(Box::new(rhs)),
            op: NodeType::Operation(Box::new(AddOp {})),
        }
    }
}

impl<T: Sub<Output = T>> Sub for BTNode<T> {
    type Output = BTNode<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        BTNode {
            left: Some(Box::new(self)),
            right: Some(Box::new(rhs)),
            op: NodeType::Operation(Box::new(SubOp {})),
        }
    }
}

impl<T: Mul<Output = T>> Mul for BTNode<T> {
    type Output = BTNode<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        BTNode {
            left: Some(Box::new(self)),
            right: Some(Box::new(rhs)),
            op: NodeType::Operation(Box::new(MulOp {})),
        }
    }
}

impl<T: Div<Output = T>> Div for BTNode<T> {
    type Output = BTNode<T>;

    fn div(self, rhs: Self) -> Self::Output {
        BTNode {
            left: Some(Box::new(self)),
            right: Some(Box::new(rhs)),
            op: NodeType::Operation(Box::new(DivOp {})),
        }
    }
}
