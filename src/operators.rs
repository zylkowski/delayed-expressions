use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub trait Operand<T>: Debug {
    fn get_value(self: Box<Self>) -> T;
}

impl<T: Debug> Operand<T> for T {
    fn get_value(self: Box<Self>) -> T {
        *self
    }
}

pub trait Op<T>: Debug {
    fn execute(&self, lhs: T, rhs: T) -> T;
}

#[derive(Debug)]
pub struct AddOp {}
impl<T: Add<Output = T>> Op<T> for AddOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

#[derive(Debug)]
pub struct SubOp {}
impl<T: Sub<Output = T>> Op<T> for SubOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs - rhs
    }
}

#[derive(Debug)]
pub struct MulOp {}
impl<T: Mul<Output = T>> Op<T> for MulOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs * rhs
    }
}

#[derive(Debug)]
pub struct DivOp {}
impl<T: Div<Output = T>> Op<T> for DivOp {
    fn execute(&self, lhs: T, rhs: T) -> T {
        lhs / rhs
    }
}
