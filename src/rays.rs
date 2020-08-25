use crate::tuples::*;

struct Ray {
    origin: Tuple,
    direction: Tuple
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
    fn computing_a_point_from_a_distance() {
        let ray = Ray{
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0)
        };
        assert_abs_diff_eq!(
            ray.position(0.0),
            point(2.0, 3.0, 4.0)
        )
    }
}