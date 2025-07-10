//! Spheres Module

use crate::material::Material;
use crate::operations::Matrix;
use crate::operations::Tuple;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f64,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new()
    }
}
impl Sphere {
    /// Constructs a new sphere with default fields
    pub fn new() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    /// Allows a transformation to be assigned to a sphere.
    pub fn set_transformation(&mut self, transformation: Matrix<4>) {
        self.transform = transformation;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
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
        let ray = ray.transform(inverse_transform);
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

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point(), "Sphere normal takes a point.");
        let inv_transform = self
            .transform
            .inverse()
            .expect("normal_at(): Could not inverse matrix.");
        let object_point = inv_transform * world_point;
        let object_normal = object_point - self.origin;
        let mut world_normal = inv_transform.transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod test {
    use crate::Color;

    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_new_sphere() {
        let origin_p = Tuple::point(0.0, 0.0, 0.0);
        let radius = 1.0;
        let s = Sphere::new();
        assert_eq!(s.origin, origin_p);
        assert_eq!(s.radius, radius);
    }

    #[test]
    fn test_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sp = Sphere::new();
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
        let mut sp = Sphere::new();
        assert_eq!(sp.transform, Matrix::identity());
        let m = Matrix::translation(2.0, 3.0, 4.0);
        sp.set_transformation(m);
        assert_eq!(sp.transform, m);
    }

    #[test]
    fn test_intersect_with_transformation() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut sp = Sphere::new();
        let m = Matrix::scaling(2.0, 2.0, 2.0);
        sp.set_transformation(m);
        let res = sp.intersect(ray);
        assert_eq!(res.len(), 2);
        assert_eq!(res.first(), Some(&3.0));
        assert_eq!(res.get(1), Some(&7.0));
    }

    #[test]
    fn test_normal_at() {
        let mut sp = Sphere::new();

        let res = sp.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(res, Tuple::vector(1.0, 0.0, 0.0));

        let res = sp.normal_at(Tuple::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            res,
            Tuple::vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
        );

        sp.set_transformation(Matrix::translation(0.0, 1.0, 0.0));
        let res = sp.normal_at(Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_eq!(res, Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn test_set_material() {
        let mut sp = Sphere::new();
        assert_eq!(sp.material, Material::new());
        let material = Material {
            color: Color::new(0.0, 1.0, 0.5),
            ambient: 1.0,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        };
        sp.set_material(material);
        assert_eq!(sp.material, material);
        sp.material.ambient = 0.5;
        assert_eq!(sp.material.ambient, 0.5);
    }
}
