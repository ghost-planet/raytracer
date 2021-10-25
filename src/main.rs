mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::rc::Rc;
use std::io;
use raytracer::vec3::{Point3, Color, Vec3};
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::camera::Camera;
use raytracer::material::{Lambertian, Metal, Dielectric};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IAMGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IAMGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLERS_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    // World
    let mut world = HittableList::default();
    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Dielectric::new(1.5));
    let material_right  = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(&Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(&Point3::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Rc::new(Sphere::new(&Point3::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.add(Rc::new(Sphere::new(&Point3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    // Camera
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            20.0, ASPECT_RATIO, aperture, dist_to_focus);

    // Render
    let mut out = io::stdout();
    raytracer::render(&world, &camera, &mut out, 
                    IAMGE_WIDTH, IMAGE_HEIGHT,
                    SAMPLERS_PER_PIXEL, MAX_DEPTH);
}

