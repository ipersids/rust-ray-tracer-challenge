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
pub mod operations;
pub mod ray;
pub mod shapes;
pub mod world;

// re-export the structures

pub use canvas::Canvas;
pub use color::Color;
pub use intersection::{Intersection, Intersections};
pub use light::Light;
pub use light::{lighting, reflect};
pub use material::Material;
pub use ray::Ray;
pub use world::{Comps, World};
