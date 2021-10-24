mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

use rand::{self,Rng};
use std::rc::Rc;
use std::io::{self, Stdout, Write};
use vec3::{Point3, Color, Vec3};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IAMGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IAMGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLERS_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(ASPECT_RATIO);

    // Render
    let mut out = io::stdout();
    // Image width height
    out.write_fmt(format_args!("P3 {} {}\n", IAMGE_WIDTH, IMAGE_HEIGHT)).unwrap();
    // 255 for max color
    out.write_fmt(format_args!("{}\n", 255)).unwrap(); 
    // RGB triplets
    let mut rng = rand::thread_rng();
    const WIDTH_FACTOR: f64 = 1.0 / (IAMGE_WIDTH as f64 - 1.0);
    const HEIGHT_FACTOR: f64 = 1.0 / (IMAGE_HEIGHT as f64 - 1.0);
    for j in (0..IMAGE_HEIGHT).rev() {
        let j = j as f64;
        for i in 0..IAMGE_WIDTH {
            let i = i as f64;
            let mut color = Color::default();
            for _ in 0..SAMPLERS_PER_PIXEL {
                let u = (i + rng.gen_range(0.0..1.0)) * WIDTH_FACTOR;
                let v = (j + rng.gen_range(0.0..1.0)) * HEIGHT_FACTOR;
                let ray = camera.gen_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&mut out, &color, SAMPLERS_PER_PIXEL).unwrap();
        }
    }
}

fn write_color(out: &mut Stdout, color: &Color, samplers_per_pixel: usize) -> io::Result<()> {
    let samplers_factor = 1.0 / (samplers_per_pixel as f64);

    let r = (255.999 * clamp((color.x() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    let g = (255.999 * clamp((color.y() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    let b = (255.999 * clamp((color.z() * samplers_factor).sqrt(), 0.0, 1.0)) as u8;
    out.write_fmt(format_args!("{} {} {}\n", r, g, b))
}

fn ray_color<T: Hittable>(ray: &Ray, hittable: &T, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(r) = hittable.hit(ray, 0.0, std::f64::MAX) {
        let target = r.p + r.normal + random_in_unit_sphere();
        return ray_color(&Ray::new(&r.p, &(target - r.p).unit_vector()), hittable, depth - 1) * 0.5;
    }

    let dir = ray.dir().unit_vector();
    let t = (dir.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
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

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}