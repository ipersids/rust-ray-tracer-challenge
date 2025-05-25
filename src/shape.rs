//! Shape Module
//!
//!

use crate::{Ray, Sphere, Tuple};

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        match self {
            Shape::Sphere(sp) => sp.intersect(ray),
        }
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shape::Sphere(sp) => sp.normal_at(point),
        }
    }
}
