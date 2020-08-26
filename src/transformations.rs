use crate::tuples::{point, Tuple};

use ndarray::prelude::*;
use ndarray::Array2;

use ndarray_linalg::solve::Inverse;

use std::f32::consts::PI;

pub type Transformation = Array2<f32>;

pub fn translation(x: f32, y: f32, z: f32) -> Transformation {
    array![
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

pub fn scale(x: f32, y: f32, z: f32) -> Transformation {
    array![
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

pub fn rotate_x(r: f32) -> Transformation {
    array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

pub fn rotate_y(r: f32) -> Transformation {
    array![
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

pub fn rotate_z(r: f32) -> Transformation {
    array![
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Transformation {
    array![
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

#[cfg(test)]
mod tests {
    use crate::transformations::*;

    #[test]
    fn translating_a_point() {
        let translation = translation(5.0, -3.0, 2.0);
        let point = point(-3.0, 4.0, 5.0);
        let transformed_point = point.transform(translation);
        assert_eq!(transformed_point.x(), 2.0);
        assert_eq!(transformed_point.y(), 1.0);
        assert_eq!(transformed_point.z(), 7.0);
    }

    #[test]

    fn translating_a_point_and_back() {
        let translation = translation(5.0, -3.0, 2.0);
        let translation_inverse = translation.inv().expect("Could not invert array");
        let point = point(-3.0, 4.0, 5.0);
        let transformed_point = point.transform(translation).transform(translation_inverse);
        assert_eq!(transformed_point.x(), -3.0);
        assert_eq!(transformed_point.y(), 4.0);
        assert_eq!(transformed_point.z(), 5.0);
    }

    #[test]

    fn applying_scaling_to_a_point() {
        let scaling = scale(-1.0, 1.0, 1.0);
        let point = point(2.0, 3.0, 4.0);
        let transformed_point = point.transform(scaling);
        assert_eq!(transformed_point.x(), -2.0);
        assert_eq!(transformed_point.y(), 3.0);
        assert_eq!(transformed_point.z(), 4.0);
    }

    #[test]
    fn rotating_a_point_in_x_axis() {
        let rotation = rotate_x(PI / 2.0);
        let point = point(0.0, 1.0, 0.0);
        let transformed_point = point.transform(rotation);
        assert_abs_diff_eq!(transformed_point.x(), 0.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 1.0);
    }

    #[test]
    fn rotating_a_point_in_y_axis() {
        let rotation = rotate_y(PI / 2.0);
        let point = point(0.0, 0.0, 1.0);
        let transformed_point = point.transform(rotation);
        assert_abs_diff_eq!(transformed_point.x(), 1.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 0.0);
    }

    #[test]
    fn rotating_a_point_in_z_axis() {
        let rotation = rotate_z(PI / 2.0);
        let point = point(0.0, 1.0, 0.0);
        let transformed_point = point.transform(rotation);
        assert_abs_diff_eq!(transformed_point.x(), -1.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 0.0);
    }

    #[test]
    fn another_rotation_in_z_axis() {
        let rotation = rotate_z(PI / 4.0);
        let point = point(0.0, 1.0, 0.0);
        let transformed_point = point.transform(rotation);
        assert_abs_diff_eq!(transformed_point.x(), -0.707106781);
        assert_abs_diff_eq!(transformed_point.y(), 0.707106781);
        assert_abs_diff_eq!(transformed_point.z(), 0.0);
    }

    #[test]
    fn shearing_x_in_proportion_to_z() {
        let shearing = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = point(2.0, 3.0, 4.0);
        let transformed_point = point.transform(shearing);
        assert_abs_diff_eq!(transformed_point.x(), 6.0);
        assert_abs_diff_eq!(transformed_point.y(), 3.0);
        assert_abs_diff_eq!(transformed_point.z(), 4.0);
    }

    #[test]
    fn shearing_y_in_proportion_to_x() {
        let shearing = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = point(2.0, 3.0, 4.0);
        let transformed_point = point.transform(shearing);
        assert_abs_diff_eq!(transformed_point.x(), 2.0);
        assert_abs_diff_eq!(transformed_point.y(), 5.0);
        assert_abs_diff_eq!(transformed_point.z(), 4.0);
    }
}
