//! Shape Module
//!
//!

use crate::{Ray, Sphere};

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
}
