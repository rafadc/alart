use crate::tuples::{point, Tuple};

use ndarray::prelude::*;
use ndarray::Array2;

use ndarray_linalg::solve::Inverse;

use std::f32::consts::PI;

fn translation(x: f32, y: f32, z: f32) -> Array2<f32> {
    array![
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn scale(x: f32, y: f32, z: f32) -> Array2<f32> {
    array![
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn rotate_x(r: f32) -> Array2<f32> {
    array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn rotate_y(r: f32) -> Array2<f32> {
    array![
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn rotate_z(r: f32) -> Array2<f32> {
    array![
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn transform(transformation: Array2<f32>, point_to_transform: Tuple) -> Tuple {
    let point_vector = array![
        [point_to_transform.x()],
        [point_to_transform.y()],
        [point_to_transform.z()],
        [point_to_transform.w()]
    ];
    let result = transformation.dot(&point_vector);
    point(result[[0, 0]], result[[1, 0]], result[[2, 0]])
}

#[cfg(test)]
mod tests {
    use crate::transformations::*;

    #[test]
    fn translating_a_point() {
        let translation = translation(5.0, -3.0, 2.0);
        let point = point(-3.0, 4.0, 5.0);
        let transformed_point = transform(translation, point);
        assert_eq!(transformed_point.x(), 2.0);
        assert_eq!(transformed_point.y(), 1.0);
        assert_eq!(transformed_point.z(), 7.0);
    }

    #[test]

    fn translating_a_point_and_back() {
        let translation = translation(5.0, -3.0, 2.0);
        let translation_inverse = translation.inv().expect("Could not invert array");
        let point = point(-3.0, 4.0, 5.0);
        let transformed_point = transform(translation, transform(translation_inverse, point));
        assert_eq!(transformed_point.x(), -3.0);
        assert_eq!(transformed_point.y(), 4.0);
        assert_eq!(transformed_point.z(), 5.0);
    }

    #[test]

    fn applying_scaling_to_a_point() {
        let scaling = scale(-1.0, 1.0, 1.0);
        let point = point(2.0, 3.0, 4.0);
        let transformed_point = transform(scaling, point);
        assert_eq!(transformed_point.x(), -2.0);
        assert_eq!(transformed_point.y(), 3.0);
        assert_eq!(transformed_point.z(), 4.0);
    }

    #[test]
    fn rotating_a_point_in_x_axis() {
        let rotation = rotate_x(PI / 2.0);
        let point = point(0.0, 1.0, 0.0);
        let transformed_point = transform(rotation, point);
        assert_abs_diff_eq!(transformed_point.x(), 0.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 1.0);
    }

    #[test]
    fn rotating_a_point_in_y_axis() {
        let rotation = rotate_y(PI / 2.0);
        let point = point(0.0, 0.0, 1.0);
        let transformed_point = transform(rotation, point);
        assert_abs_diff_eq!(transformed_point.x(), 1.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 0.0);
    }

    #[test]
    fn rotating_a_point_in_z_axis() {
        let rotation = rotate_z(PI / 2.0);
        let point = point(0.0, 1.0, 0.0);
        let transformed_point = transform(rotation, point);
        assert_abs_diff_eq!(transformed_point.x(), -1.0);
        assert_abs_diff_eq!(transformed_point.y(), 0.0);
        assert_abs_diff_eq!(transformed_point.z(), 0.0);
    }
}
