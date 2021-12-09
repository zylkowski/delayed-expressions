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

    #[test]
    fn custom_struct() {
        #[derive(Debug)]
        struct HeavyStruct {
            data: Vec<i32>,
        }

        impl Add for HeavyStruct {
            type Output = HeavyStruct;

            fn add(mut self, rhs: Self) -> Self::Output {
                self.data
                    .iter_mut()
                    .zip(rhs.data.iter())
                    .for_each(|(lhs_elem, rhs_elem)| *lhs_elem += rhs_elem);

                HeavyStruct { data: self.data }
            }
        }

        #[derive(Debug)]
        struct HeavyStructConfig {
            value: i32,
            len: usize,
        }

        impl Operand<HeavyStruct> for HeavyStructConfig {
            fn get_value(self: Box<Self>) -> HeavyStruct {
                HeavyStruct {
                    data: vec![self.value; self.len],
                }
            }
        }

        #[default_execute]
        type HSExpr = BTNode<HeavyStruct>;

        let a = HSExpr::new(HeavyStructConfig { value: 4, len: 3 })
            + HSExpr::new(HeavyStructConfig { value: 1, len: 3 });

        let a_result = a.execute();

        assert!(a_result.data == vec![5, 5, 5])
    }
}
