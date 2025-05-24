//! lighting

use crate::{Color, Material, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    intensity: Color,
    position: Tuple,
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Color) -> Self {
        assert!(position.is_point(), "Light position must be a point.");
        Self {
            intensity,
            position,
        }
    }
}

pub fn reflect(incantation: &Tuple, normal: &Tuple) -> Tuple {
    assert!(
        incantation.is_vector() && normal.is_vector(),
        "sphere.reflect(): Arguments must be a vectors."
    );
    *incantation - *normal * 2.0 * incantation.dot(normal)
}

pub fn lighting(
    material: &Material,
    light: &Light,
    position: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;
    // find the direction to the light source
    let lightv = (light.position - *position).normalize();
    // compute the ambient contribution
    let ambient = effective_color * material.ambient;
    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = lightv.dot(normalv);
    if light_dot_normal < 0.0 {
        return ambient + Color::new_black() + Color::new_black();
    }
    // compute the diffuse contribution
    let diffuse = effective_color * material.diffuse * light_dot_normal;
    // reflect_dot_eye represents the cosine of the angle between the
    // reflection vector and the eye vector. A negative number means the
    // light reflects away from the eye.
    let reflectv = reflect(&-lightv, normalv);
    let reflect_dot_eye = reflectv.dot(eyev);
    if reflect_dot_eye <= 0.0 {
        return ambient + diffuse + Color::new_black();
    }
    // compute the specular contribution
    let factor = reflect_dot_eye.powf(material.shininess);
    let specular = light.intensity * material.specular * factor;
    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_light() {
        let intensity = Color::new_black();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::point_light(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }

    #[test]
    fn test_reflect() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let res = reflect(&v, &n);
        assert_eq!(res, Tuple::vector(1.0, 1.0, 0.0));

        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let res = reflect(&v, &n);
        assert_eq!(res, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_lighting() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&Material::new(), &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        let eyev1 = Tuple::vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let result = lighting(&Material::new(), &light, &position, &eyev1, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        let light1 = Light::point_light(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&Material::new(), &light1, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        let eyev2 = Tuple::vector(0.0, -(2.0_f64.sqrt() / 2.0), -(2.0_f64.sqrt() / 2.0));
        let result = lighting(&Material::new(), &light1, &position, &eyev2, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }
}
