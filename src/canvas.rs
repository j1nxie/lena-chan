use crate::color::Color;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> &Self {
        self[(x, y)] = color;
        self
    }

    pub fn write_to_ppm(&self, path: &Path) -> std::io::Result<()> {
        let mut f = File::create(path)?;
        let headers = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut pixels = String::new();

        let mut i = 0;

        for pixel in self.pixels.iter() {
            let pixel_int = pixel.to_int(255);
            pixels.push_str(&format!("{} {} {} ", pixel_int.r, pixel_int.g, pixel_int.b));

            i += 1;

            if i == self.width {
                i = 0;
                pixels.push('\n');
            }
        }

        let mut contents = String::new();
        contents.push_str(&headers);
        contents.push_str(
            &pixels
                .trim()
                .lines()
                .map(|part| part.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
        );
        contents.push('\n');

        match f.write(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => panic!("error writing to file: {}", e),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (row, col): (usize, usize)) -> &Color {
        match self.pixels.get(row + col * self.width as usize) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for canvas size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Color {
        match self.pixels.get_mut(row + col * self.width as usize) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for canvas size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        io::{prelude::*, BufReader},
    };

    #[test]
    fn test_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for pixel in c.pixels.iter() {
            assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn test_write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let p1 = Color::new(1.0, 2.0, 3.0);
        let p2 = Color::new(2.0, 3.0, 4.0);

        c.write_pixel(3, 4, p1);
        c.write_pixel(6, 9, p2);

        assert_eq!(c[(3, 4)], p1);
        assert_eq!(c[(6, 9)], p2);
    }

    #[test]
    fn test_write_empty_ppm() {
        let c = Canvas::new(5, 3);
        c.write_to_ppm(Path::new("test_write_empty_ppm.ppm"))
            .unwrap();

        let file = File::open("test_write_empty_ppm.ppm").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        assert_eq!(content, "P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n");

        fs::remove_file("test_write_empty_ppm.ppm").unwrap();
    }

    #[test]
    fn test_write_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        c.write_to_ppm(Path::new("test_write_ppm.ppm")).unwrap();

        let file = File::open("test_write_ppm.ppm").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        assert_eq!(content, "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");

        fs::remove_file("test_write_ppm.ppm").unwrap();
    }
}
