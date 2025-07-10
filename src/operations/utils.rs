//! Utils Module
//!
//! Contains  common constants and helper functions for ray tracer.

/// Epsilon value for floating-point comparisons
pub const EPSILON: f64 = 0.00001;

/// Compare two floating-point values for approximate equality
pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

/// Cconverts degrees to radians
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_eq() {
        assert!(approx_eq(0.0, 0.0));
        assert!(approx_eq(1.0, 1.0));
        assert!(approx_eq(1.0, 1.0 + EPSILON * 0.9));
        assert!(!approx_eq(1.0, 1.0 + EPSILON * 1.1));
    }
}
