use crate::tuples::{point, Tuple};

use ndarray::prelude::*;
use ndarray::Array2;

fn translation(x: f32, y: f32, z: f32) -> Array2<f32> {
    array![
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
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
    point(result[[0,0]], result[[1,0]], result[[2,0]])
}

#[cfg(test)]
mod tests {
    use crate::transformations::*;

    #[test]
    fn translating_a_point() {
        pretty_env_logger::init();

        let translation = translation(5.0, -3.0, 2.0);
        let point = point(-3.0, 4.0, 5.0);
        let transformed_point = transform(translation, point);
        assert_eq!(transformed_point.x(), 2.0);
        assert_eq!(transformed_point.y(), 1.0);
        assert_eq!(transformed_point.z(), 7.0);
    }
}
