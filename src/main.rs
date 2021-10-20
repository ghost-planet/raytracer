mod vec3;
mod ray;

use std::io::{self, Stdout, Write};
use vec3::{Point3, Color};
use ray::Ray;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IAMGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IAMGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCOL_LENGTH: f64 = 1.0;

    let origin = Point3::default();
    let horizontal = Point3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Point3::new(0.0, 0.0, FOCOL_LENGTH);

    // Render
    let mut out = io::stdout();
    // Image width height
    out.write_fmt(format_args!("P3 {} {}\n", IAMGE_WIDTH, IMAGE_HEIGHT)).unwrap();
    // 255 for max color
    out.write_fmt(format_args!("{}\n", 255)).unwrap(); 
    // RGB triplets
    const WIDTH_FACTOR: f64 = 1.0 / (IAMGE_WIDTH as f64 - 1.0);
    const HEIGHT_FACTOR: f64 = 1.0 / (IMAGE_HEIGHT as f64 - 1.0);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IAMGE_WIDTH {
            let u = i as f64 * WIDTH_FACTOR;
            let v = j as f64 * HEIGHT_FACTOR;
            let ray = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v - origin));

            write_color(&mut out, &ray_color(&ray)).unwrap();
        }
    }
}

fn write_color(out: &mut Stdout, color: &Color) -> io::Result<()> {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;
    out.write_fmt(format_args!("{} {} {}\n", r, g, b))
}

fn ray_color(ray: &Ray) -> Color {
    let dir = ray.dir().unit_vector();
    let t = (dir.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}