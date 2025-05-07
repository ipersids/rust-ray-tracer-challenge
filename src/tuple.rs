//! # Tuple Module
//!
//! This module implements tuples, which represent either points or vectors
//! in 3D space depending on their w-coordinate.
//!

use crate::utils::equalf;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Tuple Type represents a 3D point (w=1.0) or vector (w=0.0)
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// Private constructor creates a new Tuple with given components
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a point with w-component equal 1.0
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    /// Creates a vector with w-component equal 0.0
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equalf(self.x, other.x)
            && equalf(self.y, other.y)
            && equalf(self.z, other.z)
            && equalf(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        let w_sum: f64 = self.w + other.w;

        if !equalf(w_sum, 0.0) && !equalf(w_sum, 1.0) {
            panic!("Invalid operation: Point to point addition is not allowed.")
        }
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: w_sum,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        let w_sub = self.w - other.w;

        if !equalf(w_sub, 0.0) && !equalf(w_sub, 1.0) {
            panic!("Invalid operation: subtraction a point from a vector is not allowed.")
        }

        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: w_sub,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if !equalf(self.w, 0.0) {
            panic!("Invalid operation: unary '-' operation for point is not allowed.")
        }

        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Tuple {
        if equalf(other, 0.0) {
            panic!("Invalid operation: division by zero is not allowed.")
        }

        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::EPSILON;

    #[test]
    fn test_point() {
        let res = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(res.x, 4.3);
        assert_eq!(res.y, -4.2);
        assert_eq!(res.z, 3.1);
        assert_eq!(res.w, 1.0);
    }

    #[test]
    fn test_vector() {
        let res = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(res.x, 4.3);
        assert_eq!(res.y, -4.2);
        assert_eq!(res.z, 3.1);
        assert_eq!(res.w, 0.0);
    }

    #[test]
    fn test_is_equal() {
        let a: Tuple = Tuple::point(4.3, -4.2, 3.1);
        let b: Tuple = Tuple::point(4.3 + EPSILON * 0.9, -4.2, 3.1);
        let c: Tuple = Tuple::vector(4.3, -4.2, 3.1);
        let d: Tuple = Tuple::vector(4.3 + EPSILON * 0.9, -4.2, 3.1);

        assert!(a == b);
        assert!(c == d);
        assert!(a != c);
        assert!(b != d);
    }

    #[test]
    fn test_add_valid() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v1 = Tuple::vector(-2.0, 3.0, 1.0);
        let v2 = Tuple::vector(4.0, 2.5, 6.0);

        assert!((p + v1) == Tuple::point(1.0, 1.0, 6.0));
        assert!((v1 + v2) == Tuple::vector(2.0, 5.5, 7.0));
    }

    #[test]
    #[should_panic(expected = "Invalid operation: Point to point addition is not allowed.")]
    fn test_add_invalid() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let _ = p + p;
    }

    #[test]
    fn test_sub_valid() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert!((p1 - p2) == Tuple::vector(-2.0, -4.0, -6.0));
        assert!((p1 - v2) == Tuple::point(-2.0, -4.0, -6.0));
        assert!((v1 - v2) == Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    #[should_panic(
        expected = "Invalid operation: subtraction a point from a vector is not allowed."
    )]
    fn test_sub_invalid() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let v1 = Tuple::vector(5.0, 6.0, 7.0);
        let _ = v1 - p1;
    }

    #[test]
    fn test_neg_valid() {
        let v = Tuple::vector(3.0, 2.0, 1.0);
        assert!(-v == Tuple::vector(-3.0, -2.0, -1.0));
    }

    #[test]
    #[should_panic(expected = "Invalid operation: unary '-' operation for point is not allowed.")]
    fn test_neg_invalid() {
        let v = Tuple::point(3.0, 2.0, 1.0);
        let _ = -v;
    }

    #[test]
    fn test_scalar_mul_valid() {
        let v = Tuple::vector(3.0, 2.0, 1.0);
        assert!((v * 1.0) == Tuple::vector(3.0, 2.0, 1.0));
        assert!((v * 2.0) == Tuple::vector(6.0, 4.0, 2.0));
        assert!((v * 0.5) == Tuple::vector(1.5, 1.0, 0.5));
        assert!((v * 0.0) == Tuple::vector(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_scalar_div_valid() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert!((v / 2.0) == Tuple::vector(0.5, -1.0, 1.5));
        assert!((v / 0.5) == Tuple::vector(2.0, -4.0, 6.0));
    }

    #[test]
    #[should_panic(expected = "Invalid operation: division by zero is not allowed.")]
    fn test_scalar_div_invalid() {
        let v = Tuple::vector(3.0, 2.0, 1.0);
        let _ = v / 0.0;
    }
}
