use crate::transformations::*;
use crate::tuples::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(self: &Self, r: f32) -> Tuple {
        add(&self.origin, &mul(&self.direction, r))
    }

    pub fn transform(self: &Self, transformation: Transformation) -> Ray {
        Ray {
            origin: self.origin.transform(transformation.clone()),
            direction: self.direction.transform(transformation)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::*;

    #[test]
    fn computing_a_point_from_a_0_distance() {
        let ray = Ray {
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0),
        };
        assert_abs_diff_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
    }

    #[test]
    fn computing_a_point_from_a_float_distance() {
        let ray = Ray {
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0),
        };
        assert_abs_diff_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let transformed_ray = ray.transform(translation(3.0, 4.0, 5.0));
        assert_abs_diff_eq!(transformed_ray.origin, point(4.0, 6.0, 8.0));
        assert_abs_diff_eq!(transformed_ray.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let transformed_ray = ray.transform(scale(2.0, 3.0, 4.0));
        assert_abs_diff_eq!(transformed_ray.origin, point(2.0, 6.0, 12.0));
        assert_abs_diff_eq!(transformed_ray.direction, vector(0.0, 3.0, 0.0));
    }
}
