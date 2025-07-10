//! Shape Module
//!
//!

use crate::core::Tuple;
use crate::geometry::Sphere;
use crate::graphics::Ray;
use crate::lighting::Material;

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

    pub fn get_material(&self) -> &Material {
        match self {
            Shape::Sphere(sp) => &sp.material,
        }
    }
}
