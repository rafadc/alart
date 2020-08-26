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
mod rays;
mod spheres;
mod transformations;
mod tuples;

fn main() {
    pretty_env_logger::init();

    log::info!("Staring alart");
    let mut canvas = canvas::build_canvas(100, 100);

    canvas::export_png(&canvas, "sample.png");
}
