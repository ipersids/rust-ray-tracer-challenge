//! # Tuple Module
//! 
//! This module implements tuples, which represent either points or vectors
//! in 3D space depending on their w-coordinate.
//!

/// Tuple Type represents a 3D point (w=1.0) or vector (w=0.0)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64,
}

impl Tuple {
	/// Creates a point with w-component equal 1.0
	pub fn point(x: f64, y: f64, z: f64) -> Self {
		Self { x, y, z, w: 1.0 }
	}

	/// Creates a vector with w-component equal 1.0
	pub fn vector(x: f64, y: f64, z: f64) -> Self {
		Self { x, y, z, w: 0.0 }
	}
}


// Unit tests
#[cfg(test)]
mod tests {
	use super::*;

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
}
