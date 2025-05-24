//! # Tuple Module
//!
//! This module implements tuples, which represent either points or vectors
//! in 3D space depending on their w-coordinate.
//!

use crate::utils::approx_eq;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;

/// Tuple Type represents a 3D point (w=1.0) or vector (w=0.0)
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// Tuple constructors
impl Tuple {
    /// Private constructor creates a new Tuple with given components
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a point with w-component equal to 1.0
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, POINT_W)
    }

    /// Creates a vector with w-component equal 0.0
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, VECTOR_W)
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

/// Tuple getters and helper functions
impl Tuple {
    /// Checks if given Tuple is a point
    pub fn is_point(&self) -> bool {
        approx_eq(self.w, POINT_W)
    }

    /// Checks if given Tuple is a vector
    pub fn is_vector(&self) -> bool {
        approx_eq(self.w, VECTOR_W)
    }
}

/// Tuple special math operations for vectors
impl Tuple {
    /// Calculates the squared magnitude of a 3D vector
    pub fn magnitude_squared(&self) -> f64 {
        assert!(
            self.is_vector(),
            "magnitude() called on point, expected vector"
        );
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Calculates magnitude of a 3D vector: |V| = √(x2 + y2 + z2)
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Converts an arbitrary vector to a unit vector
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    /// Calculates dot product of two vectors
    pub fn dot(&self, other: &Self) -> f64 {
        assert!(self.is_vector(), "dot() first argument must be vector");
        assert!(other.is_vector(), "dot() second argument must be vector");
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates cross product of two vectors
    pub fn cross(&self, other: &Self) -> Result<Self, &'static str> {
        if !(self.is_vector() && other.is_vector()) {
            return Err("Error: cross: Both arguments must be vectors.");
        }
        Ok(Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: VECTOR_W,
        })
    }
}

impl PartialEq for Tuple {
    /// Compares two `Tuple`s for equality
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x)
            && approx_eq(self.y, other.y)
            && approx_eq(self.z, other.z)
            && approx_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    /// Adds two `Tuple`s
    /// ## Panics
    /// Panics if adding two points.
    fn add(self, other: Self) -> Self {
        let w_sum: f64 = self.w + other.w;

        if !approx_eq(w_sum, VECTOR_W) && !approx_eq(w_sum, POINT_W) {
            panic!("Cannot add two points.")
        }
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: w_sum,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    /// Subtracts one `Tuple` from another
    /// ## Panics
    /// Panics if subtracting a point from a vector.
    fn sub(self, other: Self) -> Self {
        let w_sub = self.w - other.w;

        if !approx_eq(w_sub, VECTOR_W) && !approx_eq(w_sub, POINT_W) {
            panic!("Cannot subtract point from vector.")
        }

        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: w_sub,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    /// Negates a vector
    /// ## Panics
    /// Panics if attempting to negate a point.
    fn neg(self) -> Self {
        if !approx_eq(self.w, VECTOR_W) {
            panic!("Cannot negate a point.")
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

    /// Multiplies a `Tuple` by a scalar
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    /// Divides a `Tuple` by a scalar
    fn div(self, other: f64) -> Self {
        assert!(!approx_eq(other, 0.0), "Division by zero");

        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Display for Tuple {
    /// Formats the `Tuple` as a string
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if self.is_vector() { "Vector" } else { "Point" };
        write!(f, "{}(x: {}, y: {}, z: {})", name, self.x, self.y, self.z)
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
        assert!(res.is_point());
        assert!(!res.is_vector());
    }

    #[test]
    fn test_vector() {
        let res = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(res.x, 4.3);
        assert_eq!(res.y, -4.2);
        assert_eq!(res.z, 3.1);
        assert_eq!(res.w, 0.0);
        assert!(res.is_vector());
        assert!(!res.is_point());
    }

    #[test]
    fn test_to_array() {
        let point = Tuple::point(1.0, 2.0, 3.0);
        let arr = point.to_array();
        assert_eq!(arr[0], point.x);
        assert_eq!(arr[1], point.y);
        assert_eq!(arr[2], point.z);
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
    #[should_panic]
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
    #[should_panic]
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
    #[should_panic]
    fn test_neg_invalid() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let _ = -p;
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
    #[should_panic]
    fn test_scalar_div_invalid() {
        let v = Tuple::vector(3.0, 2.0, 1.0);
        let _ = v / 0.0;
    }

    #[test]
    fn test_magnitude_squared() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude_squared(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude_squared(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude_squared(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude_squared(), 14.0);
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude_squared(), 14.0);
    }

    #[test]
    #[should_panic]
    fn test_magnitude_squared_panic() {
        Tuple::point(-1.0, -2.0, -3.0).magnitude_squared();
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    #[should_panic]
    fn test_magnitude_panic() {
        Tuple::point(-1.0, -2.0, -3.0).magnitude();
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Tuple::vector(4.0, 0.0, 0.0).normalize(),
            Tuple::vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector(1.0, 2.0, 3.0).normalize(),
            Tuple::vector(0.26726, 0.53452, 0.80178)
        );
    }

    #[test]
    #[should_panic]
    fn test_normalize_panic() {
        Tuple::point(1.0, 2.0, 3.0).normalize();
    }

    #[test]
    fn test_dot() {
        let a: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let b: Tuple = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    #[should_panic]
    fn test_dot_panic() {
        let a: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let c: Tuple = Tuple::point(2.0, 3.0, 4.0);
        a.dot(&c);
    }

    #[test]
    fn test_cross() {
        let a: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let b: Tuple = Tuple::vector(2.0, 3.0, 4.0);
        let c: Tuple = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b).unwrap(), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a).unwrap(), Tuple::vector(1.0, -2.0, 1.0));
        assert!(b.cross(&c).is_err());
    }
}
