use transformation::rotation_y;

use crate::{canvas::Canvas, color::Color, tuple::Tuple};
use std::{f64::consts::PI, fs, path::Path, process::Command};

mod canvas;
mod color;
mod matrix;
mod transformation;
mod tuple;

fn main() {
    let mut c = Canvas::new(900, 900);
    let radius = (3.0 / 8.0) * c.width as f64;

    let center = Tuple::point(c.width as f64 / 2.0, 0.0, c.height as f64 / 2.0);
    let twelve = Tuple::point(0.0, 0.0, 1.0);

    for i in 1..12 {
        let r = rotation_y(i as f64 * PI / 6.0);
        let mut point = r * twelve;
        point.x = point.x * radius + center.x;
        point.z = point.z * radius + center.z;

        c.write_pixel(
            point.x as usize,
            point.z as usize,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    c.write_pixel(
        (twelve.x * radius + center.x) as usize,
        (twelve.z * radius + center.z) as usize,
        Color::new(1.0, 1.0, 1.0),
    );

    c.write_pixel(
        center.x as usize,
        center.z as usize,
        Color::new(1.0, 1.0, 1.0),
    );

    c.write_to_ppm(Path::new("test.ppm")).unwrap();

    Command::new("magick")
        .arg("display")
        .arg("test.ppm")
        .status()
        .expect("process failed to start");

    fs::remove_file("test.ppm").unwrap();
}
