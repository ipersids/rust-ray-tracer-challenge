use ray_tracer::core::EPSILON;
use ray_tracer::core::Tuple;
use ray_tracer::graphics::{Canvas, Color};

use std::fmt;

struct DisplayTuple(Tuple);

impl fmt::Display for DisplayTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tuple({:.2}, {:.2}, {:.2})",
            self.0.x, self.0.y, self.0.z
        )
    }
}

/// A projectile with a position (point) and velocity (vector)
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    /// Creates a new projectile
    fn new(pos: Tuple, vel: Tuple) -> Self {
        if !(pos.is_point() && vel.is_vector()) {
            panic!("Error: Projectile: invalid input.")
        }
        Self {
            position: pos,
            velocity: vel,
        }
    }

    /// Advance projectile one time step in the given environment.
    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}

/// A environment with a gravity (vector) and wind (vector)
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    /// Creates a new environment
    fn new(grav: Tuple, w: Tuple) -> Self {
        if !(grav.is_vector() && w.is_vector()) {
            panic!("Error: Environment: invalid input.")
        }
        Self {
            gravity: grav,
            wind: w,
        }
    }
}

fn main() {
    let mut proj: Projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    );
    let env: Environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0),
    );
    let mut canvas: Canvas = Canvas::new(900, 550);
    let color: Color = Color::new(0.0, 1.0, 1.0);

    println!("{:<25}  {:<25}", "Position", "Velocity");
    println!("{:-<24}  {:->25}", "", "");
    while proj.position.y > EPSILON {
        println!(
            "{:<25}  {:<25}",
            DisplayTuple(proj.position),
            DisplayTuple(proj.velocity)
        );
        let y: f64 = canvas.height as f64 - proj.position.y;
        if y < 0.0 {
            break;
        }
        canvas.add_pixel(proj.position.x.round() as usize, y.round() as usize, color);
        proj.tick(&env);
    }
    println!("Writing into file './renders/chapter01.ppm'");
    let _ = canvas.to_ppm(Some("chapter01.ppm".to_owned()));
}
