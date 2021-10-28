use std::fs::File;
use std::rc::Rc;
use rand::{self,Rng};
use clap::{Arg, App};
use raytracer::vec3::{Point3, Color, Vec3};
use raytracer::hittable::{Hittable, BVH};
use raytracer::sphere::{Sphere, AnimatedSphere};
use raytracer::camera::Camera;
use raytracer::material::{Lambertian, Metal, Dielectric, DiffuseLight};
use raytracer::texture::{SolidTexture, CheckerTexture, ImageTexture};
use raytracer::rect::AARect;
use raytracer::aabox::AABox;

fn main() {
    let is_number = |v: String| {
        match v.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("The value cannot convert to usize"))
        }
    };

    let matches = App::new("My Super Program")
                            .version("0.1.0")
                            .author("VincentGong. <return0xffff@gmail.com>")
                            .about("A simple ray tracer")
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
                            .arg(Arg::with_name("SCENE")
                                .long("scene")
                                .value_name("SCENE")
                                .help("Scene to render (earch | random | light | cornell_box), default is random")
                                .takes_value(true))
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

    let scene = matches.value_of("SCENE").unwrap_or("random");
    
    // World and Camera
    let mut rng = rand::thread_rng();
    let (world, camera) = match scene {
        "earch" => earch_scene(&mut rng, aspect_ratio),
        "light" => light_scene(&mut rng, aspect_ratio),
        "cornell_box" => cornell_box_scene(&mut rng, aspect_ratio),
        _ => random_scene(&mut rng, aspect_ratio),
    };

    // Render
    let mut out = File::create(output).unwrap();
    raytracer::render(&world, &camera, &mut out, 
                    image_width, image_height,
                    samplers_per_pixel, max_depth);
}

fn random_scene<T: Rng>(rng: &mut T, aspect_ratio: f64) -> (BVH, Camera) {
    // World
    let mut world = Vec::<Rc<dyn Hittable>>::new();

    let odd = Rc::new(SolidTexture::new(&Color::new(0.2, 0.3, 0.1)));
    let even = Rc::new(SolidTexture::new(&Color::new(0.9, 0.9, 0.9)));
    let ground_material = Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(odd, even))));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

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
                let sphere_material = Rc::new(Lambertian::new(Rc::new(SolidTexture::new(&albedo))));
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                world.push(Rc::new(AnimatedSphere::new(&center, &center1, 1.0, 0.2, sphere_material)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_in(0.5, 1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_material = Rc::new(Metal::new(Rc::new(SolidTexture::new(&albedo)), fuzz));
                world.push(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            } else {
                // glass
                let sphere_material = Rc::new(Dielectric::new(1.5));
                world.push(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            }
        }
    }

    let material = Rc::new(Dielectric::new(1.5));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, material)));

    let material = Rc::new(Lambertian::new(Rc::new(SolidTexture::new(&Color::new(0.4, 0.2, 0.1)))));
    world.push(Rc::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, material)));

    let material = Rc::new(Metal::new(Rc::new(SolidTexture::new(&Color::new(0.7, 0.6, 0.5))), 0.0));
    world.push(Rc::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, material)));

    let world = BVH::new(world);

    // Camera 
    const DIST_TO_FOCUS: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const SHUTTER_DURATION: f64 = 1.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            20.0, aspect_ratio, APERTURE, DIST_TO_FOCUS, SHUTTER_DURATION);
    
    (world, camera)
}

fn earch_scene<T: Rng>(_rng: &mut T, aspect_ratio: f64) -> (BVH, Camera) {
    // World
    let mut world = Vec::<Rc<dyn Hittable>>::new();

    let texture = Rc::new(ImageTexture::new("assets/earthmap.jpg"));
    let material = Rc::new(Lambertian::new(texture));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, 0.0, 0.0), 2.0, material)));

    let texture = Rc::new(SolidTexture::new(&Color::new(4.0, 4.0, 4.0)));
    let material = Rc::new(DiffuseLight::new(texture));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, 0.0, 3.5), 1.0, material)));
    let world = BVH::new(world);

    // Camera 
    const DIST_TO_FOCUS: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const SHUTTER_DURATION: f64 = 1.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            20.0, aspect_ratio, APERTURE, DIST_TO_FOCUS, SHUTTER_DURATION);

    (world, camera)
}

fn light_scene<T: Rng>(_rng: &mut T, aspect_ratio: f64) -> (BVH, Camera) {
    let mut world = Vec::<Rc<dyn Hittable>>::new();

    let texture = Rc::new(SolidTexture::new(&Color::new(0.0, 1.0, 0.0)));
    let material = Rc::new(Lambertian::new(texture));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, material)));
    let texture = Rc::new(SolidTexture::new(&Color::new(1.0, 0.0, 0.0)));
    let material = Rc::new(Lambertian::new(texture));
    world.push(Rc::new(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, material)));

    let texture = Rc::new(SolidTexture::new(&Color::new(4.0, 4.0, 4.0)));
    let material = Rc::new(DiffuseLight::new(texture));
    world.push(Rc::new(AARect::new_xy(3.0, 5.0, 1.0, 3.0, -2.0, material)));
    let world = BVH::new(world);

    // Camera 
    const DIST_TO_FOCUS: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const SHUTTER_DURATION: f64 = 1.0;
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            20.0, aspect_ratio, APERTURE, DIST_TO_FOCUS, SHUTTER_DURATION);

    (world, camera)
}

fn cornell_box_scene<T: Rng>(_rng: &mut T, aspect_ratio: f64) -> (BVH, Camera) {
    let mut world = Vec::<Rc<dyn Hittable>>::new();

    let texture = Rc::new(SolidTexture::new(&Color::new(0.65, 0.05, 0.05)));
    let red = Rc::new(Lambertian::new(texture));
    let texture = Rc::new(SolidTexture::new(&Color::new(0.73, 0.73, 0.73)));
    let white = Rc::new(Lambertian::new(texture));
    let texture = Rc::new(SolidTexture::new(&Color::new(0.12, 0.45, 0.15)));
    let green = Rc::new(Lambertian::new(texture));
    let texture = Rc::new(SolidTexture::new(&Color::new(15.0, 15.0, 15.0)));
    let light = Rc::new(DiffuseLight::new(texture));

    // Walls
    world.push(Rc::new(AARect::new_yz(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())));
    world.push(Rc::new(AARect::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    world.push(Rc::new(AARect::new_xz(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.push(Rc::new(AARect::new_xz(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(Rc::new(AARect::new_xy(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    // Light
    world.push(Rc::new(AARect::new_xz(213.0, 343.0, 227.0, 332.0, 554.0, light)));

    // Blocks
    world.push(Rc::new(AABox::new(&Point3::new(130.0, 0.0, 65.0), &Point3::new(295.0, 165.0, 230.0), white.clone())));
    world.push(Rc::new(AABox::new(&Point3::new(265.0, 0.0, 295.0), &Point3::new(430.0, 330.0, 460.0), white.clone())));

    let world = BVH::new(world);

    // Camera 
    const DIST_TO_FOCUS: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const SHUTTER_DURATION: f64 = 1.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(&look_from,
                            &look_at,
                            &up,
                            40.0, aspect_ratio, APERTURE, DIST_TO_FOCUS, SHUTTER_DURATION);

    (world, camera)
} 