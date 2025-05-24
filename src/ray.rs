//! Ray Module

use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Result<Self, &'static str> {
        match origin.is_point() && direction.is_vector() {
            true => Ok(Ray { origin, direction }),
            false => Err("Ray origin should be a point and direction - a vector."),
        }
    }
}

impl Ray {
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ray() {
        let point: Tuple = Tuple::point(1.0, 2.0, 3.0);
        let vec: Tuple = Tuple::vector(4.0, 5.0, 6.0);
        let res: Result<Ray, &'static str> = Ray::new(point, vec);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.origin, point);
        assert_eq!(res.direction, vec);
        let res = Ray::new(vec, vec);
        assert!(res.is_err());
        let res = Ray::new(point, point);
        assert!(res.is_err());
        let res = Ray::new(vec, point);
        assert!(res.is_err());
    }

    #[test]
    fn test_position() {
        let point: Tuple = Tuple::point(2.0, 3.0, 4.0);
        let vec: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let ray: Ray = Ray::new(point, vec).unwrap();
        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}
