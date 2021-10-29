pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod bbox;
pub mod texture;
pub mod rect;
pub mod aabox;
pub mod medium;

use rand::{self,Rng};
use pbr::ProgressBar;
use std::io::{self, Write};

use vec3::Color;
use ray::Ray;
use hittable::Hittable;
use camera::Camera;

pub fn render<T: Hittable, W: Write>(world: &T, camera: &Camera, out: &mut W,
                            image_width: usize, image_height: usize, 
                            samplers_per_pixel: usize, max_depth: usize) {
    // Render
    let background_color = Color::default();

    let mut pb = ProgressBar::new((image_width * image_height) as u64);
    pb.message("Rendering ");
    // Image width height
    out.write_fmt(format_args!("P3 {} {}\n", image_width, image_height)).unwrap();
    // 255 for max color
    out.write_fmt(format_args!("{}\n", 255)).unwrap(); 
    // RGB triplets
    let mut rng = rand::thread_rng();
    let width_factor: f64 = 1.0 / (image_width as f64 - 1.0);
    let height_factor: f64 = 1.0 / (image_height as f64 - 1.0);
    for j in (0..image_height).rev() {
        let j = j as f64;
        for i in 0..image_width {
            let i = i as f64;
            let mut color = Color::default();
            for _ in 0..samplers_per_pixel {
                let u = (i + rng.gen_range(0.0..1.0)) * width_factor;
                let v = (j + rng.gen_range(0.0..1.0)) * height_factor;
                let ray = camera.gen_ray(u, v);
                color += ray_color(&ray, &background_color, world, max_depth);
            }
            write_color(out, &color, samplers_per_pixel).unwrap();
            pb.inc();
        }
    }
    pb.finish();
}

fn write_color<W: Write>(out: &mut W, color: &Color, samplers_per_pixel: usize) -> io::Result<()> {
    let samplers_factor = 1.0 / (samplers_per_pixel as f64);

    let r = (255.999 * clamp((color.x() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    let g = (255.999 * clamp((color.y() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    let b = (255.999 * clamp((color.z() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    out.write_fmt(format_args!("{} {} {}\n", r, g, b))
}

fn ray_color<T: Hittable>(ray: &Ray, background_color: &Color, hittable: &T, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(r) = hittable.hit(ray, 0.0001, std::f64::MAX) {
        let emit = r.material.emitted(r.u, r.v, &r.p);
        if let Some((attenuation, ray)) = r.material.scatter(ray, &r) {
            emit + ray_color(&ray, background_color, hittable, depth - 1) * attenuation
        } else {
            emit
        }
    } else {
        *background_color
    }
}

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}