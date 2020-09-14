#[macro_use]
extern crate approx;
extern crate log;
extern crate pretty_env_logger;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate openblas_src;

mod canvas;
mod colors;
mod intersections;
mod lights;
mod materials;
mod rays;
mod spheres;
mod transformations;
mod tuples;

use crate::colors::Color;
use crate::intersections::hit;
use crate::lights::Light;
use crate::materials::*;
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::tuples::*;

fn main() {
    pretty_env_logger::init();

    log::info!("Staring alart");
    let mut canvas = canvas::build_canvas(1000, 1000);

    let mut sphere = Sphere::new();
    sphere.transformation = transformations::translation(0.0, 0.0, 3.0);
    sphere.material = Material::new();
    sphere.material.color = Color::new(1.0, 0.2, 1.0);

    let light = Light {
        position: point(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    };

    for i in 0..1000 {
        for j in 0..1000 {
            let ray = Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(-0.5 + (i as f32) * 0.001, 0.5 - (j as f32) * 0.001, 1.0)
                    .normalize(),
            };

            let maybe_hit_with_sphere = hit(sphere.intersect(&ray));

            if maybe_hit_with_sphere.is_some() {
                let hit_with_sphere = maybe_hit_with_sphere.unwrap();
                let point = ray.position(hit_with_sphere.t);
                let normal_of_hit = hit_with_sphere.object.normal_at(&point);

                let eye = ray.direction.negate();

                let color = lighting(
                    hit_with_sphere.object.material,
                    light.clone(),
                    &point,
                    &eye,
                    &normal_of_hit,
                );

                canvas::write_pixel(&mut canvas, i, j, color);
            }
        }
    }

    canvas::export_png(&canvas, "sample.png");
}
