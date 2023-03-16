use float_eq::{derive_float_eq, float_eq};
use std::f64::{self, EPSILON};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive_float_eq(
    ulps_tol = "TupleUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "TupleDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    pub fn cross(&self, other: &Tuple) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let cmp = Tuple {
            x: 1.0 * EPSILON,
            y: 1.0 * EPSILON,
            z: 1.0 * EPSILON,
            w: 1.0 * EPSILON,
        };

        float_eq!(self, other, abs <= cmp)
    }
}

impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Tuple {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Tuple> for Tuple {
    type Output = f64;

    fn mul(self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn test_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn test_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn test_new_point() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_new_vector() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_cmp_point() {
        let a = Tuple::point(1.0, 1.0, 2.0);
        let b = Tuple::point(1.0, 1.0, 2.0);
        let c = Tuple::point(1.0, 1.0, 1.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_cmp_vector() {
        let a = Tuple::vector(1.0, 1.0, 2.0);
        let b = Tuple::vector(1.0, 1.0, 2.0);
        let c = Tuple::vector(1.0, 1.0, 1.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_cmp_point_vector() {
        let a = Tuple::point(1.0, 1.0, 2.0);
        let b = Tuple::vector(1.0, 1.0, 2.0);
        assert_ne!(a, b);
    }

    #[test]
    fn test_add_tuple() {
        let a = Tuple::point(1.0, 1.0, 2.0);
        let b = Tuple::point(1.0, 1.0, 2.0);
        let result = Tuple::new(2.0, 2.0, 4.0, 2.0);

        assert_eq!(a + b, result);
    }

    #[test]
    fn test_sub_tuple() {
        let a = Tuple::point(1.0, 1.0, 2.0);
        let b = Tuple::point(1.0, 1.0, 2.0);
        let result = Tuple::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(a - b, result);
    }

    #[test]
    fn test_neg_tuple() {
        let t = Tuple::point(1.0, 1.0, 2.0);
        let result = Tuple::new(-1.0, -1.0, -2.0, -1.0);

        assert_eq!(-t, result);
    }

    #[test]
    fn test_scalar_mul() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 3.5;
        let result = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(a * scalar, result);
    }

    #[test]
    fn test_scalar_div() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 2.0;
        let result = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a / scalar, result);
    }

    #[test]
    fn test_magnitude() {
        let a = Tuple::vector(1.0, 0.0, 0.0);
        let b = Tuple::vector(-1.0, -2.0, -3.0);

        assert_float_eq!(a.magnitude(), 1.0, abs <= EPSILON);
        assert_float_eq!(b.magnitude(), (14.0_f64).sqrt(), abs <= EPSILON);
    }

    #[test]
    fn test_normalize() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let normalized_v = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), normalized_v);
    }

    #[test]
    fn test_vector_mul() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let result = 20.0;

        assert_float_eq!(a * b, result, abs <= EPSILON)
    }

    #[test]
    fn test_cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let result_ab = Tuple::vector(-1.0, 2.0, -1.0);
        let result_ba = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(a.cross(&b), result_ab);
        assert_eq!(b.cross(&a), result_ba);
    }
}
