use ray_tracer::core::{Matrix, Tuple};
use ray_tracer::geometry::Sphere;
use ray_tracer::graphics::{Canvas, Color, Ray};
use std::f64::consts::PI;

fn main() {
    let ray_orig = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let red = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();

    // shrink it along the y axis
    // shape.set_transformation(Matrix::scaling(1.0, 0.5, 1.0));
    // shrink it along the x axis
    // shape.set_transformation(Matrix::scaling(0.5, 1.0, 1.0));
    //shrink it, and rotate it!
    shape.set_transformation(Matrix::rotation_z(PI / 4.0) * Matrix::scaling(0.5, 1.0, 1.0));
    // shrink it, and skew it!
    // shape.set_transformation(
    //     Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scaling(0.5, 1.0, 1.0),
    // );

    for y in 0..100 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..100 {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let direction = position - ray_orig;
            let ray = Ray::new(ray_orig, direction.normalize());
            let xs = shape.intersect(ray);

            let hit = xs.iter().any(|item| *item >= 0.0);
            if hit {
                canvas.add_pixel(x, y, red);
            }
        }
    }

    println!("Writing into file './renders/chapter05.ppm'");
    let _ = canvas.to_ppm(Some("chapter05.ppm".to_owned()));
}
