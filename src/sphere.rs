//! Spheres Module

use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Tuple, radius: f64) -> Self {
        assert!(origin.is_point(), "Sphere origin must be a point.");
        assert!(radius > 0.0, "Sphere radius must be positive.");

        Sphere { origin, radius }
    }
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction).unwrap();
        let b = 2.0 * ray.direction.dot(&sphere_to_ray).unwrap();
        let c = sphere_to_ray.dot(&sphere_to_ray).unwrap() - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        vec![
            (-b - (discriminant.sqrt())) / (2.0 * a),
            (-b + (discriminant.sqrt())) / (2.0 * a),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_sphere() {
        let origin_p = Tuple::point(0.0, 0.0, 1.0);
        let radius = 100.0;
        let s = Sphere::new(origin_p, radius);
        assert_eq!(s.origin, origin_p);
        assert_eq!(s.radius, radius);
    }

    #[test]
    #[should_panic]
    fn test_new_sphere_invalid_origin() {
        let origin_v = Tuple::vector(0.0, 0.0, 1.0);
        let radius = 100.0;
        let _ = Sphere::new(origin_v, radius);
    }

    #[test]
    #[should_panic]
    fn test_new_sphere_invalid_radius() {
        let origin = Tuple::point(0.0, 0.0, 1.0);
        let radius = -10.0;
        let _ = Sphere::new(origin, radius);
    }

    #[test]
    fn test_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let sp = Sphere::new(Tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&4.0));
        assert_eq!(xs.get(1), Some(&6.0));
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&5.0));
        assert_eq!(xs.get(1), Some(&5.0));
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 0);
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&-1.0));
        assert_eq!(xs.get(1), Some(&1.0));
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&-6.0));
        assert_eq!(xs.get(1), Some(&-4.0));
    }
}
