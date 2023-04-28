use crate::matrix::Matrix;

// TODO: implement a fluent API for this
// e.g. transform = matrix.rotate_x().scale().translate();
// basically letting u chain method calls instead of just functions.
// good exercise for refactoring :p
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

pub fn rotation_y(angle: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(0, 0)] = angle.cos();
    matrix[(0, 2)] = angle.sin();
    matrix[(2, 0)] = -angle.sin();
    matrix[(2, 2)] = angle.cos();

    matrix
}

pub fn rotation_z(angle: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(0, 0)] = angle.cos();
    matrix[(0, 1)] = -angle.sin();
    matrix[(1, 0)] = angle.sin();
    matrix[(1, 1)] = angle.cos();

    matrix
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut matrix = Matrix::identity_matrix(4);
    matrix[(0, 1)] = xy;
    matrix[(0, 2)] = xz;
    matrix[(1, 0)] = yx;
    matrix[(1, 2)] = yz;
    matrix[(2, 0)] = zx;
    matrix[(2, 1)] = zy;

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

    // FIXME: this fails due to floating point imprecision and all of that.
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
            Tuple::point(
                0.0,
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0
            )
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
            Tuple::point(
                0.0,
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
                -(2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
            )
        );
    }

    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
                0.0,
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
            )
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(
                -(2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
                (2.0_f64.sqrt() / 2.0 * 100000.0).round() / 100000.0,
                0.0,
            )
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_xy() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_xz() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_yx() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_yz() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_zx() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_zy() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_transform_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        let p3 = b * p2;
        let p4 = c * p3;

        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_transform_sequence_chained() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
