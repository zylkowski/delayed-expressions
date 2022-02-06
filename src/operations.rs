use serde::{Deserialize, Serialize};

use crate::btree_internal::BtNode;

use crate::operators::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use std::convert::Infallible;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum NodeType<Operator, Value>
where
    Operator: ?Sized,
    Value: ?Sized,
{
    Operation(Box<Operator>),
    Val(Box<Value>),
}

impl<Operator, Value> Serialize for NodeType<Operator, Value>
where
    Operator: ?Sized,
    Value: ?Sized,
    Box<Operator>: Serialize,
    Box<Value>: Serialize,
{
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

// pub type DelayedExpressionNode<Operator, Value> = NodeType<Operator, Value>;
pub type OpTree<Operator, Value> = BtNode<NodeType<Operator, Value>>;

impl<Operator, Value> Serialize for OpTree<Operator, Value>
where

    NodeType<Operator, Value>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
// impl<Result, Operator, Value> OpTree<Operator, Value>
// where
//     Operator: Op<Result>,
//     Value: Op<Result>,
// {
// }

// impl<Result> OpTree<Result> {
//     pub fn new(val: impl Operand<Result> + 'static) -> Self {
//         OpTree {
//             left: None,
//             right: None,
//             op: NodeType::Val(Box::new(val)),
//         }
//     }

//     pub fn op(
//         operator: impl Op<Result> + 'static,
//         lhs: OpTree<Result>,
//         rhs: OpTree<Result>,
//     ) -> Self {
//         OpTree {
//             left: Some(Box::new(lhs)),
//             right: Some(Box::new(rhs)),
//             op: NodeType::Operation(Box::new(operator)),
//         }
//     }
// }

// impl<Result: Add<Output = Result>> Add for OpTree<Result> {
//     type Output = OpTree<Result>;

//     fn add(self, rhs: Self) -> Self::Output {
//         OpTree {
//             left: Some(Box::new(self)),
//             right: Some(Box::new(rhs)),
//             op: NodeType::Operation(Box::new(AddOp {})),
//         }
//     }
// }

// impl<Result: Sub<Output = Result>> Sub for OpTree<Result> {
//     type Output = OpTree<Result>;

//     fn sub(self, rhs: Self) -> Self::Output {
//         OpTree {
//             left: Some(Box::new(self)),
//             right: Some(Box::new(rhs)),
//             op: NodeType::Operation(Box::new(SubOp {})),
//         }
//     }
// }

// impl<Result: Mul<Output = Result>> Mul for OpTree<Result> {
//     type Output = OpTree<Result>;

//     fn mul(self, rhs: Self) -> Self::Output {
//         OpTree {
//             left: Some(Box::new(self)),
//             right: Some(Box::new(rhs)),
//             op: NodeType::Operation(Box::new(MulOp {})),
//         }
//     }
// }

// impl<Result: Div<Output = Result>> Div for OpTree<Result> {
//     type Output = OpTree<Result>;

//     fn div(self, rhs: Self) -> Self::Output {
//         OpTree {
//             left: Some(Box::new(self)),
//             right: Some(Box::new(rhs)),
//             op: NodeType::Operation(Box::new(DivOp {})),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{DelayedExpressionNode, NodeType, OpTree};

//     #[test]
//     fn match_node() {
//         // let a: DelayedExpressionNode<i32>;

//         // match a {
//         //     NodeType::Operation(_) => todo!(),
//         //     NodeType::Val(_) => todo!(),
//         // }
//     }
// }
