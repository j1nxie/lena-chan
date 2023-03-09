use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width as usize]; height as usize],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Self {
        self.pixels[x][y] = color;
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for (i, row) in c.pixels.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                assert_eq!(c.pixels[i][j], Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let p1 = Color::new(1.0, 2.0, 3.0);
        let p2 = Color::new(2.0, 3.0, 4.0);

        c.write_pixel(3, 4, p1);
        c.write_pixel(6, 9, p2);

        assert_eq!(c.pixels[3][4], p1);
        assert_eq!(c.pixels[6][9], p2);
    }
}
