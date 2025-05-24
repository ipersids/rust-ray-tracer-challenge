//! # Ray Module
//!
//! This module defines the `Ray` struct and associated methods for representing
//! and manipulating rays in 3D space. A ray consists of an origin point and a direction vector,
//! and is commonly used in ray tracing to determine intersections with objects.

use crate::tuple::Tuple;

/// Represents a ray in 3D space, defined by an origin point and a direction vector.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(
            origin.is_point() && direction.is_vector(),
            "Ray origin must be a point and direction - a vector."
        );
        Ray { origin, direction }
    }
}

impl Ray {
    /// Computes the position along the ray at distance `t`.
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ray() {
        let point: Tuple = Tuple::point(1.0, 2.0, 3.0);
        let vec: Tuple = Tuple::vector(4.0, 5.0, 6.0);
        let res: Ray = Ray::new(point, vec);
        assert_eq!(res.origin, point);
        assert_eq!(res.direction, vec);
    }

    #[test]
    #[should_panic]
    fn test_new_ray_invalid_origin() {
        let vec: Tuple = Tuple::vector(4.0, 5.0, 6.0);
        let _ = Ray::new(vec, vec);
    }

    #[test]
    #[should_panic]
    fn test_new_ray_invalid_dir() {
        let point: Tuple = Tuple::point(1.0, 2.0, 3.0);
        let _ = Ray::new(point, point);
    }

    #[test]
    fn test_position() {
        let point: Tuple = Tuple::point(2.0, 3.0, 4.0);
        let vec: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let ray: Ray = Ray::new(point, vec);
        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}
