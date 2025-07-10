//! World Module

use crate::operations::{Matrix, Tuple};
use crate::shapes::{Shape, Sphere};
use crate::{Color, Intersection, Intersections, Light, Ray, lighting};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Shape>,
    pub light: Light,
}

pub struct Comps {
    pub t: f64,
    pub obj: Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub inside: bool,
    pub normalv: Tuple,
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}

impl World {
    pub fn new() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let light = Light::point_light(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        Self {
            objects: vec![Shape::Sphere(s1), Shape::Sphere(s2)],
            light,
        }
    }
}

impl World {
    pub fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut collect: Vec<Intersection> = vec![];
        for (i, shape) in self.objects.iter().enumerate() {
            let xs = shape.intersect(ray);
            if !xs.is_empty() {
                for t in &xs {
                    collect.push(Intersection::new(*t, i));
                }
            }
        }
        Intersections::from(collect)
    }

    pub fn prepare_computations(&self, intersection: &Intersection, ray: Ray) -> Comps {
        let obj = self.objects.get(intersection.shape_id).unwrap();
        let t = intersection.t;
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let mut normalv = obj.normal_at(point);
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        Comps {
            t,
            obj: obj.clone(),
            point,
            eyev,
            inside,
            normalv,
        }
    }

    pub fn shade_hit(&self, comps: Comps) -> Color {
        lighting(
            comps.obj.get_material(),
            &self.light,
            &comps.point,
            &comps.eyev,
            &comps.normalv,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let collection = self.intersect_world(ray);
        let closest_hit = collection.hit();
        if closest_hit.is_none() {
            return Color::new_black();
        }
        let comps = self.prepare_computations(closest_hit.unwrap(), ray);
        self.shade_hit(comps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 2);

        let Shape::Sphere(s1) = w.objects.first().unwrap();
        assert_eq!(s1.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(s1.radius, 1.0);
        assert_eq!(s1.material.color, Color::new(0.8, 1.0, 0.6));
        assert_eq!(s1.material.diffuse, 0.7);
        assert_eq!(s1.material.specular, 0.2);

        let Shape::Sphere(s2) = w.objects.get(1).unwrap();
        assert_eq!(s2.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(s2.radius, 1.0);
        assert_eq!(s2.transform, Matrix::scaling(0.5, 0.5, 0.5));

        assert_eq!(w.light.intensity, Color::new(1.0, 1.0, 1.0));
        assert_eq!(w.light.position, Tuple::point(-10.0, 10.0, -10.0));
    }

    #[test]
    fn test_intersect_world() {
        let world = World::new();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let collect: Intersections = world.intersect_world(ray);
        assert_eq!(collect.count_items(), 4);
        assert_eq!(collect.collection.first().map(|item| item.t), Some(4.0));
        assert_eq!(collect.collection.get(1).map(|item| item.t), Some(4.5));
        assert_eq!(collect.collection.get(2).map(|item| item.t), Some(5.5));
        assert_eq!(collect.collection.get(3).map(|item| item.t), Some(6.0));
    }

    #[test]
    fn test_prepare_computations() {
        let world = World::new();
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(1.0, 0);
        let comps = world.prepare_computations(&i, ray);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn test_color_at() {
        let mut world = World::new();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let color = world.color_at(ray);
        assert_eq!(color, Color::new_black());
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let color = world.color_at(ray);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
        if let Some(Shape::Sphere(outer)) = world.objects.first_mut() {
            outer.material.ambient = 1.0;
        }
        let mut inner_color = Color::new_white();
        if let Some(Shape::Sphere(inner)) = world.objects.get_mut(1) {
            inner.material.ambient = 1.0;
            inner_color = inner.material.color;
        }
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let color = world.color_at(ray);
        assert_eq!(color, inner_color);
    }
}
