#[allow(unused)]
use ray_tracer::operations::Tuple;
use ray_tracer::{Canvas, Color};

fn main() {
    let p = Tuple::point(1.0, 2.0, 3.0);
    println!("{}", p);
    let v = Tuple::vector(1.0, 2.0, 3.0);
    println!("{}", v);
    let c = Color::new(1.0, 0.8, 0.6);
    println!("{}", c);

    let canvas = Canvas::new_with_color(10, 2, c);
    if let Err(msg) = canvas.to_ppm(Some("chapter02.ppm".to_owned())) {
        eprintln!("Error: {msg}");
    }
}
