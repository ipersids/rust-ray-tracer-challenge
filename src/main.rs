#[allow(unused)]
use ray_tracer::Tuple;

fn main() {
    let p = Tuple::point(1.0, 2.0, 3.0);
    println!("Point: x={}, y={}, z={}, w={}", p.x, p.y, p.z, p.w);
    let v = Tuple::vector(1.0, 2.0, 3.0);
    println!("Vector: x={}, y={}, z={}, w={}", v.x, v.y, v.z, v.w);
}
