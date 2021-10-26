mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::rc::Rc;
use std::io;
use rand::{self,Rng};
use raytracer::vec3::{Point3, Color, Vec3};
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::camera::Camera;
use raytracer::material::{Lambertian, Metal, Dielectric};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IAMGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IAMGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLERS_PER_PIXEL: usize = 500;
    const MAX_DEPTH: usize = 50;

    // World
    let mut rng = rand::thread_rng();
    let world = random_scene(&mut rng);

    // Camera
    const DIST_TO_FOCUS: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            20.0, ASPECT_RATIO, APERTURE, DIST_TO_FOCUS);

    // Render
    let mut out = io::stdout();
    raytracer::render(&world, &camera, &mut out, 
                    IAMGE_WIDTH, IMAGE_HEIGHT,
                    SAMPLERS_PER_PIXEL, MAX_DEPTH);
}

fn random_scene<T: Rng>(rng: &mut T) -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let center = Point3::new(a + 0.9 * rng.gen_range(0.0..1.0), 0.2, b + 0.9 * rng.gen_range(0.0..1.0));
            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let choose_mat = rng.gen_range(0.0..1.0);
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                let sphere_material = Rc::new(Lambertian::new(&albedo));
                world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_in(0.5, 1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            } else {
                // glass
                let sphere_material = Rc::new(Dielectric::new(1.5));
                world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            }
        }
    }

    let material = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, material)));

    let material = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, material)));

    let material = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, material)));

    world
}