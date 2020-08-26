use std::cmp::Ordering;

use crate::spheres::Sphere;

pub struct Intersection {
    pub t: f32,
    pub object: Sphere
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let positive_intersections = intersections.into_iter().filter(|x| x.t > 0.0);
    positive_intersections.min_by(|x,y| x.t.partial_cmp(&y.t).unwrap_or(Ordering::Equal))
}

#[cfg(test)]
mod tests {
    use crate::intersections::*;

    #[test]
    fn finds_hit_on_all_intersections_positive() {
        let intersections = vec![
            Intersection{
                t: 1.0,
                object: Sphere{}
            },
            Intersection{
                t: 3.0,
                object: Sphere{}
            }
        ];
        assert_eq!(
            hit(intersections).unwrap().t,
            1.0);
    }

    #[test]
    fn finds_hit_on_some_intersections_negative() {
        let intersections = vec![
            Intersection{
                t: 1.0,
                object: Sphere{}
            },
            Intersection{
                t: -1.0,
                object: Sphere{}
            }
        ];
        assert_eq!(
            hit(intersections).unwrap().t,
            1.0);
    }

    #[test]
    fn does_not_find_hit_on_all_intersections_negative() {
        let intersections = vec![
            Intersection{
                t: -4.0,
                object: Sphere{}
            },
            Intersection{
                t: -1.0,
                object: Sphere{}
            }
        ];
        assert!(hit(intersections).is_none());
    }
}