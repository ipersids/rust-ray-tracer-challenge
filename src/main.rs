#[allow(unused)]
use ray_tracer::Tuple;

fn main() {
    let p = Tuple::point(1.0, 2.0, 3.0);
    println!("{}", p);
    let v = Tuple::vector(1.0, 2.0, 3.0);
    println!("{}", v);
}
