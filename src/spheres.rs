use crate::intersections::Intersection;
use crate::rays::Ray;
use crate::transformations::*;
use crate::tuples::*;
use crate::materials::Material;

use ndarray_linalg::solve::Inverse;

#[derive(PartialEq, Clone, Debug)]
pub struct Sphere {
    pub transformation: Transformation,
    pub material: Material
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transformation: identity(),
            material: Material::new()
        }
    }

    pub fn intersect(self: &Self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(
            self.transformation
                .inv()
                .expect("Couldn't invert transformation to apply to ray"),
        );
        log::debug!(
            "Ray: {:?} and inverted: {:?}",
            ray.clone(),
            transformed_ray.clone()
        );
        let sphere_to_ray = sub(&transformed_ray.origin, &point(0.0, 0.0, 0.0));
        let a = dot(&transformed_ray.direction, &transformed_ray.direction);
        let b = dot(&transformed_ray.direction, &sphere_to_ray) * 2.0;
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![
            Intersection {
                t: t1,
                object: self.clone(),
            },
            Intersection {
                t: t2,
                object: self.clone(),
            },
        ]
    }

    pub fn normal_at(self: &Self, world_point: Tuple) -> Tuple {
        let transformation = self.transformation.inv().expect("Could not invert sphere transform");
        let object_point = world_point.transform(transformation);
        let object_normal = sub(&object_point, &point(0.0, 0.0, 0.0));
        let world_transformation = self.transformation.inv().expect("Could not invert sphere transform").t().to_owned();
        let mut world_normal = object_normal.transform(world_transformation);
        world_normal.w = 0.0;
        world_normal
    }
}

#[cfg(test)]
mod tests {
    use crate::spheres::*;

    #[test]
    fn the_default_transformation_for_a_sphere_is_the_identity() {
        let sphere = Sphere::new();
        assert_eq!(sphere.transformation, identity());
    }

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

    #[test]
    fn intersecting_a_scaled_shpere_with_a_ray() {
        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut sphere = Sphere::new();
        sphere.transformation = scale(2.0, 2.0, 2.0);
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn normal_at_x_axis() {
        let sphere = Sphere::new();
        assert_abs_diff_eq!(
            sphere.normal_at(point(1.0, 0.0, 0.0)),
            vector(1.0, 0.0, 0.0)
        )
    }

    #[test]
    fn normal_at_y_axis() {
        let sphere = Sphere::new();
        assert_abs_diff_eq!(
            sphere.normal_at(point(0.0, 1.0, 0.0)),
            vector(0.0, 1.0, 0.0)
        )
    }

    #[test]
    fn normal_at_z_axis() {
        let sphere = Sphere::new();
        assert_abs_diff_eq!(
            sphere.normal_at(point(0.0, 0.0, 1.0)),
            vector(0.0, 0.0, 1.0)
        )
    }

    #[test]
    fn normal_at_non_axial_point() {
        let sphere = Sphere::new();
        assert_abs_diff_eq!(
            sphere.normal_at(point(
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0
            )),
            vector(
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0
            )
        )
    }

    #[test]
    fn normal_at_a_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.transformation = translation(0.0, 1.0, 0.0);
        assert_abs_diff_eq!(
            sphere.normal_at(point(0.0, 1.70711, -0.70711)),
            vector(0.0, 0.70711, -0.70711)
        )
    }
}
