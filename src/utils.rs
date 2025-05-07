//! Utils Module
//!
//! Contains  common constants and helper functions for ray tracer.

/// Epsilon value for floating-point comparisons
pub const EPSILON: f64 = 0.00001;

/// Compare two floating-point values for approximate equality
pub fn equalf(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equalf() {
        assert!(equalf(0.0, 0.0));
        assert!(equalf(1.0, 1.0));
        assert!(equalf(1.0, 1.0 + EPSILON * 0.9));
        assert!(!equalf(1.0, 1.0 + EPSILON * 1.1));
    }
}
