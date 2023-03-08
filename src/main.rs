#[derive(Clone, Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}