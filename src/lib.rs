//! # Ray Tracer Challenge Implementation
//!
//! This crate implements a ray tracer following "The Ray Tracer Challenge"
//! book by Jamis Buck.

// expose these modules

pub mod canvas;
pub mod color;
pub mod tuple;
pub mod utils;

// re-export the structures

pub use canvas::Canvas;
pub use color::Color;
pub use tuple::Tuple;
pub use utils::EPSILON;
pub use utils::approx_eq;
