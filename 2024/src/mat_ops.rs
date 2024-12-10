use crate::Mat;
/// Basic matrix arithmetic operations for Mat
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! mat_scalar_op_impl {
    ($name:ident, $name_lower:ident, $name_assign:ident, $name_assign_lower:ident, $op:tt) => {
        impl $name<i64> for Mat {
            type Output = Mat;

            fn $name_lower(self, rhs: i64) -> Self::Output {
                let lhs = self.data();
                let dest: Vec<i64> = lhs.iter().map(|a| *a $op rhs).collect();

                Mat::from_shape_vec(self.shape(), dest)
            }
        }

        impl $name<i64> for &Mat {
            type Output = Mat;

            fn $name_lower(self, rhs: i64) -> Self::Output {
                let lhs = self.data();
                let dest: Vec<i64> = lhs.iter().map(|a| *a $op rhs).collect();

                Mat::from_shape_vec(self.shape(), dest)
            }
        }

        impl $name_assign<i64> for Mat {
            fn $name_assign_lower(&mut self, rhs: i64) {
                let lhs = self.data_mut();
                for l in lhs.iter_mut() {
                    *l = *l $op rhs;
                }
            }
        }
    };
}

macro_rules! mat_elementwise_op_impl {
    ($name:ident, $name_lower:ident, $name_assign:ident, $name_assign_lower:ident, $op:tt, $error:tt) => {

        impl $name<&Mat> for &Mat {
            type Output = Mat;

            fn $name_lower(self, rhs: &Mat) -> Self::Output {
                let lhs = self.data();
                let rhs = rhs.data();
                if lhs.len() != rhs.len() {
                    panic!($error);
                }

                let dest: Vec<i64> = lhs.iter().zip(rhs.iter()).map(|(a, b)| *a $op *b).collect();

                Mat::from_shape_vec(self.shape(), dest)
            }
        }

        impl $name_assign<&Mat> for Mat {
            fn $name_assign_lower(&mut self, rhs: &Mat) {
                let lhs = self.data_mut();
                let rhs = rhs.data();
                if lhs.len() != rhs.len() {
                    panic!($error);
                }

                for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
                    *l = *l $op *r;
                }
            }
        }
    };
}

mat_scalar_op_impl!(Add, add, AddAssign, add_assign, +);
mat_scalar_op_impl!(Sub, sub, SubAssign, sub_assign, -);
mat_scalar_op_impl!(Mul, mul, MulAssign, mul_assign, *);
mat_scalar_op_impl!(Div, div, DivAssign, div_assign, /);

mat_elementwise_op_impl!(Add, add, AddAssign, add_assign, +, "matrices must have the same shape for element-wise addition");
mat_elementwise_op_impl!(Sub, sub, SubAssign, sub_assign, -, "matrices must have the same shape for element-wise subtraction");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_elementwise_addition() {
        let a = Mat::from_shape_vec((2, 2), vec![1, 2, 3, 4]);
        let b = Mat::from_shape_vec((2, 2), vec![5, 6, 7, 8]);

        assert_eq!(&a + &b, Mat::from_shape_vec((2, 2), vec![6, 8, 10, 12]));
    }

    #[test]
    pub fn test_elementwise_addition_assign() {
        let mut a = Mat::from_shape_vec((2, 2), vec![1, 2, 3, 4]);
        let b = Mat::from_shape_vec((2, 2), vec![5, 6, 7, 8]);

        a += &b;

        assert_eq!(a, Mat::from_shape_vec((2, 2), vec![6, 8, 10, 12]));
    }

    #[test]
    pub fn test_scalar_add() {
        let a = Mat::from_shape_vec((2, 2), vec![1, 2, 3, 4]);
        let b = 5;

        assert_eq!(&a + b, Mat::from_shape_vec((2, 2), vec![6, 7, 8, 9]));
    }

    #[test]
    pub fn test_scalar_add_assign() {
        let mut a = Mat::from_shape_vec((2, 2), vec![1, 2, 3, 4]);
        let b = 5;

        a += b;

        assert_eq!(a, Mat::from_shape_vec((2, 2), vec![6, 7, 8, 9]));
    }
}
