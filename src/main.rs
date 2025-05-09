use ray_tracer::Color;
#[allow(unused)]
use ray_tracer::Tuple;

fn main() {
    let p = Tuple::point(1.0, 2.0, 3.0);
    println!("{}", p);
    let v = Tuple::vector(1.0, 2.0, 3.0);
    println!("{}", v);
    let c = Color::new(0.9, 0.6, 0.75);
    println!("{}", c);
}
