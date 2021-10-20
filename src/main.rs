mod vec3;
mod ray;

use std::io::{self, Stdout, Write};
use vec3::Color;

fn main() {
    // Image

    const IAMGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render

    // Image width height
    println!("P3 {} {}", IAMGE_WIDTH, IMAGE_HEIGHT);
    // 255 for max color
    println!("{}", 255);
    // RGB triplets
    const WIDTH_FACTOR: f64 = 1.0 / (IAMGE_WIDTH as f64 - 1.0);
    const HEIGHT_FACTOR: f64 = 1.0 / (IMAGE_HEIGHT as f64 - 1.0);
    let mut out = io::stdout();
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IAMGE_WIDTH {
            let r = i as f64 * WIDTH_FACTOR;
            let g = j as f64 * HEIGHT_FACTOR;
            let b = 0.25;

            write_color(&mut out, &Color::new(r, g, b)).unwrap();
        }
    }
}

fn write_color(out: &mut Stdout, color: &Color) -> io::Result<()> {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;
    out.write_fmt(format_args!("{} {} {}\n", r, g, b))
}