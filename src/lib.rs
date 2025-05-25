//! # Ray Tracer Challenge Implementation
//!
//! This crate implements a ray tracer following "The Ray Tracer Challenge"
//! book by Jamis Buck.

// expose these modules

pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrixes;
pub mod ray;
pub mod shape;
pub mod sphere;
pub mod tuple;
pub mod utils;

// re-export the structures

pub use canvas::Canvas;
pub use color::Color;
pub use intersection::{Intersection, Intersections};
pub use light::Light;
pub use light::{lighting, reflect};
pub use material::Material;
pub use matrixes::Matrix;
pub use ray::Ray;
pub use sphere::Sphere;
pub use tuple::Tuple;
pub use utils::EPSILON;
pub use utils::{approx_eq, deg_to_rad};
