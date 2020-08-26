use crate::rays::Ray;
use crate::tuples::*;
use crate::intersections::Intersection;
use crate::transformations::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Sphere {
    transformation: Transformation
}

impl Sphere {
    pub fn new() -> Sphere{
        Sphere {
            transformation: identity()
        }
    }

    pub fn intersect(self: &Self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = sub(&ray.origin, &point(0.0, 0.0, 0.0));
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&ray.direction, &sphere_to_ray) * 2.0;
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![
            Intersection{
                t: t1,
                object: self.clone()
            },
            Intersection {
                t: t2,
                object: self.clone()
            }
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::spheres::*;

    #[test]
    fn a_ray_intersects_a_sphere_in_two_points() {
        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[0].object, sphere);
        assert_eq!(intersections[1].t, 6.0);
        assert_eq!(intersections[1].object, sphere);
    }

    #[test]
    fn a_ray_is_tangent_to_a_sphere() {
        let ray = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[0].object, sphere);

        assert_eq!(intersections[1].t, 5.0);
        assert_eq!(intersections[1].object, sphere);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[0].object, sphere);

        assert_eq!(intersections[1].t, 1.0);
        assert_eq!(intersections[1].object, sphere);
    }

    #[test]
    fn the_sphere_is_behind_the_ray() {
        let ray = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[0].object, sphere);

        assert_eq!(intersections[1].t, -4.0);
        assert_eq!(intersections[1].object, sphere);
    }
}
