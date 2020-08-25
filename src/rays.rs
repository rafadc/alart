use crate::tuples::*;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    pub fn position(self: &Self, r: f32) -> Tuple {
        add(&self.origin, &mul(&self.direction, r))
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::*;

    #[test]
    fn computing_a_point_from_a_0_distance() {
        let ray = Ray{
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0)
        };
        assert_abs_diff_eq!(
            ray.position(0.0),
            point(2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn computing_a_point_from_a_float_distance() {
        let ray = Ray{
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0)
        };
        assert_abs_diff_eq!(
            ray.position(2.5),
            point(4.5, 3.0, 4.0)
        );
    }
}