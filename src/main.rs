use crate::{canvas::Canvas, color::Color, tuple::Tuple};
use std::path::Path;

mod canvas;
mod color;
mod matrix;
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
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);

    while projectile.position.y > 0.0 {
        projectile.tick(&environment);
        let color = Color::new(1.0, 0.0, 0.0);

        if projectile.position.y > 0.0 {
            println!(
                "drawing at coord: ({}, {})",
                projectile.position.x as usize, projectile.position.y as usize,
            );
            c.write_pixel(
                projectile.position.x as usize,
                projectile.position.y as usize,
                color,
            );
        }
    }

    c.write_to_ppm(Path::new("test.ppm")).unwrap();
}
