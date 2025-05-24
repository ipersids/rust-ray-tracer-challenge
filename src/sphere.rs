//! Spheres Module

use crate::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f64,
    pub transform: Matrix<4>,
}

impl Sphere {
    /// Constructs a new sphere with a specified center and radius.
    /// Ensures the center is a valid point and the radius is greater than zero.
    pub fn new(origin: Tuple, radius: f64) -> Self {
        assert!(origin.is_point(), "Sphere origin must be a point.");
        assert!(radius > 0.0, "Sphere radius must be positive.");
        let transform = Matrix::identity();
        Sphere {
            origin,
            radius,
            transform,
        }
    }

    /// Allows a transformationto be assigned to a sphere.
    pub fn set_transformation(&mut self, transformation: Matrix<4>) {
        self.transform = transformation;
    }
}

impl Sphere {
    /// Determines where a given ray intersects the sphere, if at all.
    /// Returns a list of distances along the ray for each intersection point.
    /// The result may be empty if there are no intersections.
    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        let inverse_transform = self
            .transform
            .inverse()
            .expect("Sphere transform must be invertible");
        let ray = Ray::new(
            inverse_transform * ray.origin,
            inverse_transform * ray.direction,
        );
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius.powi(2);
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
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sp = Sphere::new(Tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&4.0));
        assert_eq!(xs.get(1), Some(&6.0));
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&5.0));
        assert_eq!(xs.get(1), Some(&5.0));
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 0);
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&-1.0));
        assert_eq!(xs.get(1), Some(&1.0));
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = sp.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.first(), Some(&-6.0));
        assert_eq!(xs.get(1), Some(&-4.0));
    }

    #[test]
    fn test_set_transformation() {
        let mut sp = Sphere::new(Tuple::point(0.0, 0.0, 0.0), 1.0);
        assert_eq!(sp.transform, Matrix::identity());
        let m = Matrix::translation(2.0, 3.0, 4.0);
        sp.set_transformation(m);
        assert_eq!(sp.transform, m);
    }

    #[test]
    fn test_intersect_with_transformation() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut sp = Sphere::new(Tuple::point(0.0, 0.0, 0.0), 1.0);
        let m = Matrix::scaling(2.0, 2.0, 2.0);
        sp.set_transformation(m);
        let res = sp.intersect(ray);
        assert_eq!(res.len(), 2);
        assert_eq!(res.first(), Some(&3.0));
        assert_eq!(res.get(1), Some(&7.0));
    }
}
