#[macro_use]
extern crate approx;
extern crate pretty_env_logger;
extern crate log;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate openblas_src;

mod canvas;
mod colors;
mod transformations;
mod tuples;

fn main() {
    pretty_env_logger::init();

    log::info!("Staring alart");
    let mut canvas = canvas::build_canvas(20,10);
    for i in 0..19 {
        for j in 0..4 {
            canvas::write_pixel(&mut canvas, i, j, colors::Color{r: 1.0, g: 0.0, b: 0.0});
        }
    }
    log::debug!("canvas is = {:?}", canvas);

    canvas::export_png(&canvas, "sample.png");
}
