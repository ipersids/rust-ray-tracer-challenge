//! # Color Module

use crate::operations::Tuple;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    tuple: Tuple,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            tuple: Tuple::vector(r, g, b),
        }
    }

    pub fn new_black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn new_white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl Color {
    pub fn get_clamped_red_u8(&self) -> u8 {
        (self.tuple.x.clamp(0.0, 1.0) * 255.0).floor() as u8
    }

    pub fn get_clamped_green_u8(&self) -> u8 {
        (self.tuple.y.clamp(0.0, 1.0) * 255.0).floor() as u8
    }

    pub fn get_clamped_blue_u8(&self) -> u8 {
        (self.tuple.z.clamp(0.0, 1.0) * 255.0).floor() as u8
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self {
        Self {
            tuple: self.tuple + other.tuple,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self {
        Self {
            tuple: self.tuple - other.tuple,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self {
        Self {
            tuple: self.tuple * other,
        }
    }
}

/// Hadamard product
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self {
        Self {
            tuple: Tuple::vector(
                self.tuple.x * other.tuple.x,
                self.tuple.y * other.tuple.y,
                self.tuple.z * other.tuple.z,
            ),
        }
    }
}

impl PartialEq for Color {
    /// Compares two `Color`s for equality
    fn eq(&self, other: &Self) -> bool {
        self.tuple == other.tuple
    }
}

impl std::fmt::Display for Color {
    /// Formats the `Color` as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color(r: {}, g: {}, b: {})",
            self.tuple.x, self.tuple.y, self.tuple.z
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let res = Color::new(-0.5, 0.4, 1.7);
        let black = Color::new_black();
        let white = Color::new_white();
        assert_eq!(res.tuple.x, -0.5);
        assert_eq!(res.tuple.y, 0.4);
        assert_eq!(res.tuple.z, 1.7);
        assert_eq!(black, Color::new(0.0, 0.0, 0.0));
        assert_eq!(white, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_is_equal() {
        let a: Color = Color::new(4.3, -4.2, 3.1);
        let b: Color = Color::new(0.3, 4.2, -3.3);

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_add() {
        let a: Color = Color::new(0.9, 0.6, 0.75);
        let b: Color = Color::new(0.7, 0.1, 0.25);
        let res: Color = Color::new(1.6, 0.7, 1.0);

        assert_eq!(a + b, res);
    }

    #[test]
    fn test_sub() {
        let a: Color = Color::new(0.9, 0.6, 0.75);
        let b: Color = Color::new(0.7, 0.1, 0.25);
        let res: Color = Color::new(0.2, 0.5, 0.5);

        assert_eq!(a - b, res);
    }

    #[test]
    fn test_mul_by_scalar() {
        let a: Color = Color::new(0.2, 0.3, 0.4);
        let res: Color = Color::new(0.4, 0.6, 0.8);

        assert_eq!(a * 2.0, res);
    }

    #[test]
    fn test_mul_by_color() {
        let a: Color = Color::new(1.0, 0.2, 0.4);
        let b: Color = Color::new(0.9, 1.0, 0.1);
        let res: Color = Color::new(0.9, 0.2, 0.04);

        assert_eq!(a * b, res);
    }

    #[test]
    fn test_get_clamped_u8() {
        let px = Color::new(1.0, 0.8, 0.6);
        let red = px.get_clamped_red_u8();
        let green = px.get_clamped_green_u8();
        let blue = px.get_clamped_blue_u8();

        assert_eq!(red, 255);
        assert_eq!(green, 204);
        assert_eq!(blue, 153);
    }
}
