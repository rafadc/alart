extern crate image;

use image::{ImageBuffer, RgbImage};

use crate::colors::Color;

#[derive(Debug)]
pub struct Canvas {
    height: u32,
    width: u32,
    data: Vec<Vec<Color>>,
}

pub fn build_canvas(width: u32, height: u32) -> Canvas {
    Canvas {
        width: width,
        height: height,
        data: vec![
            vec![
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0
                };
                width as usize
            ];
            height as usize
        ],
    }
}

fn pixel_at(canvas: &Canvas, x: u32, y: u32) -> Color {
    canvas.data[y as usize][x as usize].clone()
}

pub fn write_pixel(canvas: &mut Canvas, x: u32, y: u32, color: Color) {
    canvas.data[y as usize][x as usize] = color.clone();
}

pub fn export_png(canvas: &Canvas, filename: &str) {
    let width = canvas.width.clone();
    let height = canvas.height.clone();
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = pixel_at(canvas, i, j);
            let r = (255.0 * pixel.r) as u8;
            let g = (255.0 * pixel.g) as u8;
            let b = (255.0 * pixel.b) as u8;

            log::debug!("Pixel {} becomes {}", pixel.r, r);

            img.put_pixel(i, j, image::Rgb([r, g, b]));
        }
    }

    img.save(filename).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::canvas::*;

    #[test]
    fn default_color_is_black() {
        let canvas = build_canvas(10, 10);
        assert_eq!(
            pixel_at(&canvas, 0, 0),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0
            }
        )
    }

    #[test]
    fn can_write_pixels_in_canvas() {
        let color = Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        };
        let mut canvas = build_canvas(10, 10);
        write_pixel(&mut canvas, 0, 0, color);
        assert_eq!(
            pixel_at(&canvas, 0, 0),
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0
            }
        )
    }
}
