use float_eq::{derive_float_eq, float_eq};
use std::f64::{self, EPSILON};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive_float_eq(
    ulps_tol = "ColorUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "ColorDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let cmp = Color {
            r: 1.0 * EPSILON,
            g: 1.0 * EPSILON,
            b: 1.0 * EPSILON,
        };

        float_eq!(self, other, abs <= cmp)
    }
}

impl Eq for Color {}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn test_add_color() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(1.6, 0.7, 1.0);

        assert_eq!(a + b, result);
    }

    #[test]
    fn test_sub_color() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(0.2, 0.5, 0.5);

        assert_eq!(a - b, result);
    }

    #[test]
    fn test_mul_color_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let scalar = 2.0;
        let result = Color::new(0.4, 0.6, 0.8);

        assert_eq!(c * scalar, result);
    }

    #[test]
    fn test_hadamard_product_color() {
        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);
        let result = Color::new(0.9, 0.2, 0.04);

        assert_eq!(a * b, result);
    }
}
