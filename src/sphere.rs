use crate::ray::Ray;

pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Sphere
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sphere() {
        let sphere = Sphere::new();
    }
}
