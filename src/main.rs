use crate::tuple::Tuple;

mod canvas;
mod color;
mod tuple;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn tick(&mut self, environment: &Environment) {
        *self = Self {
            position: self.position + self.velocity,
            velocity: self.velocity + environment.gravity + environment.wind,
        }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        println!("current y position: {}", projectile.position.y);
        projectile.tick(&environment);
    }
}
