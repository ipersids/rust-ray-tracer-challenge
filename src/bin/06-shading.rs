use ray_tracer::operations::Tuple;
use ray_tracer::{Canvas, Color, Light, Ray, Sphere, lighting};

fn main() {
    let ray_orig = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 400.0;
    let canvas_size = canvas_pixels as usize;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);

    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1.0, 0.2, 0.5);
    sphere.material.shininess = 90.0;
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Light::point_light(light_position, light_color);

    // sphere.set_transformation(Matrix::scaling(1.0, 0.5, 1.0));

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let direction = position - ray_orig;
            let ray = Ray::new(ray_orig, direction.normalize());
            let xs = sphere.intersect(ray);

            let hit = xs.iter().find(|item| **item >= 0.0);
            if hit.is_some() {
                let point = ray.position(*hit.unwrap());
                let normal = sphere.normal_at(point);
                let eye = -ray.direction;
                let final_color = lighting(&sphere.material, &light, &point, &eye, &normal);
                canvas.add_pixel(x, y, final_color);
            }
        }
    }

    println!("Writing into file './renders/chapter05.ppm'");
    let _ = canvas.to_ppm(Some("chapter05.ppm".to_owned()));
}
