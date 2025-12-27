use crate::math::{Matrix, Tuple};

#[derive(Debug, PartialEq)]
pub struct Camera {
    position: Tuple,
    target: Tuple,
    fov: f64,
    inverse_view_transform: Matrix<4>,
}

impl Camera {
    pub fn new(position: Tuple, target: Tuple, fov: f64) -> Self {
        let default_up = Tuple::vector(0.0, 1.0, 0.0);
        let transform = Matrix::<4>::view_transform(position, target, default_up);
        let inverse_transform = transform.inverse().unwrap_or(Matrix::<4>::identity());
        Self {
            position,
            target,
            fov,
            inverse_view_transform: inverse_transform,
        }
    }

    pub fn get_position(&self) -> Tuple {
        self.position
    }

    pub fn get_forward(&self) -> Tuple {
        (self.target - self.position).normalize()
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }

    pub fn get_inverse_view_transform(&self) -> Matrix<4> {
        self.inverse_view_transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_camera_ok() {
        let position = Tuple::point(0.0, 0.0, 8.0);
        let target = Tuple::point(0.0, 0.0, 0.0);
        let fov = 60.0;
        let camera = Camera::new(position, target, fov);
        let expected = Camera {
            position: Tuple::point(0.0, 0.0, 8.0),
            target: Tuple::point(0.0, 0.0, 0.0),
            fov: 60.0,
            inverse_view_transform: Matrix::<4>::translation(0.0, 0.0, -8.0)
                .inverse()
                .unwrap_or(Matrix::<4>::identity()),
        };
        assert_eq!(camera, expected);
    }
}
