#![allow(dead_code)]
#![allow(unused_imports)]
use delayed_expressions_derive::*;
pub mod operators;
use operators::*;
pub mod btree;
use btree::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Mul};

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

    #[test]
    fn custom_operator() {
        #[derive(Debug)]
        struct AddNTimesOp {
            n: usize,
        }
        impl<T: Add<Output = T> + Copy> Op<T> for AddNTimesOp {
            fn execute(&self, lhs: T, rhs: T) -> T {
                (0..self.n).fold(lhs, |lhs, _| lhs + rhs)
            }
        }

        let a = Integer::op(AddNTimesOp { n: 3 }, Integer::new(5), Integer::new(2));
        let a_result = a.execute();

        assert!(a_result == 11);
    }
}
