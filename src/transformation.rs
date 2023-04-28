use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(0, 3)] = x;
    matrix[(1, 3)] = y;
    matrix[(2, 3)] = z;

    matrix
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(0, 0)] = x;
    matrix[(1, 1)] = y;
    matrix[(2, 2)] = z;

    matrix
}

pub fn rotation_x(angle: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(1, 1)] = angle.cos();
    matrix[(1, 2)] = -angle.sin();
    matrix[(2, 1)] = angle.sin();
    matrix[(2, 2)] = angle.cos();

    matrix
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn test_mul_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_mul_translation_matrix_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inverse * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_vectors_unaffected_by_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_mul_scaling_matrix_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_mul_scaling_matrix_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_mul_scaling_matrix_inverse() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inverse * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotation_x_inverse() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inverse = half_quarter.inverse();

        assert_eq!(
            inverse * p,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }
}
