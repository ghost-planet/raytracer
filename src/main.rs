mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::fs::File;
use std::rc::Rc;
use rand::{self,Rng};
use clap::{Arg, App};
use raytracer::vec3::{Point3, Color, Vec3};
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::camera::Camera;
use raytracer::material::{Lambertian, Metal, Dielectric};

fn main() {
    let is_number = |v: String| {
        match v.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("The value cannot convert to usize"))
        }
    };

    let matches = App::new("My Super Program")
                            .version("1.0")
                            .author("Kevin K. <kbknapp@gmail.com>")
                            .about("Does awesome things")
                            .arg(Arg::with_name("WIDTH")
                                .short("w")
                                .long("width")
                                .value_name("WIDTH")
                                .help("Sets output file width")
                                .takes_value(true)
                                .required(true)
                                .validator(is_number))
                            .arg(Arg::with_name("HEIGHT")
                                .short("h")
                                .long("height")
                                .value_name("HEIGHT")
                                .help("Sets output file height")
                                .takes_value(true)
                                .required(true)
                                .validator(is_number))
                            .arg(Arg::with_name("SAMPLERS")
                                .short("s")
                                .long("samplers")
                                .value_name("SAMPLERS")
                                .help("Sets samplers per pixel, default is 500")
                                .takes_value(true)
                                .validator(is_number))
                            .arg(Arg::with_name("DEPTH")
                                .short("d")
                                .long("depth")
                                .value_name("DEPTH")
                                .help("Sets max ray trace depth, default is 50")
                                .takes_value(true)
                                .validator(is_number))
                            .arg(Arg::with_name("OUTPUT")
                                .help("Sets the output file to use")
                                .required(true)
                                .index(1))
                            .get_matches();

    // Image
    let output = matches.value_of("OUTPUT").unwrap();

    let image_width = matches.value_of("WIDTH").unwrap().parse::<usize>().unwrap();
    let image_height = matches.value_of("HEIGHT").unwrap().parse::<usize>().unwrap();
    let aspect_ratio = image_width as f64 / image_height as f64;

    let samplers_per_pixel = matches.value_of("SAMPLERS").unwrap_or("500").parse::<usize>().unwrap();
    let max_depth = matches.value_of("DEPTH").unwrap_or("50").parse::<usize>().unwrap();

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
                            20.0, aspect_ratio, APERTURE, DIST_TO_FOCUS);

    // Render
    let mut out = File::create(output).unwrap();
    raytracer::render(&world, &camera, &mut out, 
                    image_width, image_height,
                    samplers_per_pixel, max_depth);
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