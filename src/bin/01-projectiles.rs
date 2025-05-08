use ray_tracer::tuple::Tuple;
use ray_tracer::utils::EPSILON;

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
        Tuple::vector(1.0, 1.0, 0.0).normalize(),
    );
    let env: Environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0),
    );

    // Simulate the projectile's motion until it hits the ground
    println!("{:<25}  {:<25}", "Position", "Velocity");
    println!("{:-<24}  {:->25}", "", "");

    while proj.position.y > EPSILON {
        // two columns, each using our Display impl
        println!("{:<25}  {:<25}", proj.position, proj.velocity);
        proj.tick(&env);
    }
}
