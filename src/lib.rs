#![allow(dead_code)]
#![allow(unused_imports)]
use delayed_expressions_derive::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

trait Operand<T>: Debug {
    fn get_value(self: Box<Self>) -> T;
}

impl<T: Debug> Operand<T> for T {
    fn get_value(self: Box<Self>) -> T {
        *self
    }
}

trait Op<T>: Debug {
    fn execute(&self, lhs: T, rhs: T) -> T;
}

#[derive(Debug)]
struct AddOp {}
impl<T: Add<Output = T>> Op<T> for AddOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

#[derive(Debug)]
struct SubOp {}
impl<T: Sub<Output = T>> Op<T> for SubOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs - rhs
    }
}

#[derive(Debug)]
struct MulOp {}
impl<T: Mul<Output = T>> Op<T> for MulOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs * rhs
    }
}

#[derive(Debug)]
struct DivOp {}
impl<T: Div<Output = T>> Op<T> for DivOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs / rhs
    }
}

#[derive(Debug)]
enum NodeType<T> {
    Operation(Box<dyn Op<T>>),
    Val(Box<dyn Operand<T>>),
}

trait ExpressionExecute<T> {
    fn execute(self) -> T;
}

type ChildNode<T> = Option<Box<BTNode<T>>>;
#[derive(Debug)]
struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: NodeType<T>,
}

impl<T> BTNode<T> {
    fn new(val: impl Operand<T> + 'static) -> Self {
        BTNode {
            left: None,
            right: None,
            op: NodeType::Val(Box::new(val)),
        }
    }

    fn op(operator: impl Op<T> + 'static, lhs: BTNode<T>, rhs: BTNode<T>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[default_execute]
    type Integer = BTNode<i32>;

    #[test]
    fn execution_test() {
        let a = Integer::op(AddOp {}, Integer::new(5), Integer::new(7));
        let b = Integer::new(5) + Integer::new(7);

        let a_result = a.execute();
        let b_result = b.execute();

        assert!(a_result == b_result);
        assert!(a_result == 12);
    }
}
