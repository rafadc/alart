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
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::tuples::*;

fn main() {
    pretty_env_logger::init();

    log::info!("Staring alart");
    let mut canvas = canvas::build_canvas(1000, 1000);

    let mut sphere = Sphere::new();
    sphere.transformation = transformations::translation(0.0, 0.0, 3.0);
    for i in 0..1000 {
        for j in 0..1000 {
            let ray = Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(-0.5 + (i as f32) * 0.001, -0.5 + (j as f32) * 0.001, 1.0),
            };
            if hit(sphere.intersect(&ray)).is_some() {
                canvas::write_pixel(
                    &mut canvas,
                    i,
                    j,
                    Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                    },
                );
            }
        }
    }

    canvas::export_png(&canvas, "sample.png");
}
