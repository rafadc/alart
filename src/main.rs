#[macro_use]
extern crate approx;
extern crate log;
extern crate pretty_env_logger;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate openblas_src;

mod canvas;
mod colors;
mod rays;
mod spheres;
mod transformations;
mod tuples;

use std::f32::consts::PI;

use transformations::*;

fn main() {
    pretty_env_logger::init();

    log::info!("Staring alart");
    let mut canvas = canvas::build_canvas(100, 100);

    let point_center = tuples::point(0.0, 0.0, 0.0);
    let move_upwards_in_clock = translation(0.0, -40.0, 0.0);
    let translation_to_canvas = translation(50.0, 50.0, 0.0);

    for hour in 0..12 {
        let rotation = rotate_z((PI / 6.0) * hour as f32);
        let new_point = transform(
            translation_to_canvas.clone(),
            transform(
                rotation,
                transform(move_upwards_in_clock.clone(), point_center.clone()),
            ),
        );

        log::debug!("Writing point: {:?}", new_point);

        canvas::write_pixel(
            &mut canvas,
            new_point.x() as u32,
            new_point.y() as u32,
            colors::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        );
    }

    canvas::export_png(&canvas, "sample.png");
}
