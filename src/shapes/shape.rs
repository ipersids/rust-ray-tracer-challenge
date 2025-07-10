//! Shape Module
//!
//!

use crate::operations::Tuple;
use crate::shapes::Sphere;
use crate::{Material, Ray};

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
