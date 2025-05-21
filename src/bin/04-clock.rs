use std::f64::consts::PI;

use ray_tracer::matrixes::Matrix;
use ray_tracer::tuple::Tuple;
use ray_tracer::{Canvas, Color};

fn main() {
    let mut canvas: Canvas = Canvas::new(640, 640);
    let color: Color = Color::new(0.0, 1.0, 1.0);

    let p = Tuple::point(0.0, 1.0, 0.0);
    let radius = 200.0;

    for clock in 0..12 {
        let angle = clock as f64 * PI / 6.0;
        let transform = Matrix::rotation_z(angle) * Matrix::scaling(radius, radius, 0.0);
        let res = transform * p;
        let x = (canvas.width as f64 / 2.0 + res.x.round()) as usize;
        let y = (canvas.height as f64 / 2.0 - res.y.round()) as usize;
        canvas.add_pixel(x, y, color);
    }
    println!("Writing into file './renders/chapter04.ppm'");
    let _ = canvas.to_ppm(Some("chapter04.ppm".to_owned()));
}
