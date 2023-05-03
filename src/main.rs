use std::io::{self, Write};
mod camera;
mod hittable;
mod ray;
mod rtweekend;
mod vec3;
use camera::camera::Camera;
use ray::ray::Ray;
use rtweekend::rtweekend::random_f32;
use std::f32::INFINITY;
use vec3::{color::Color, point::Point3d};

use crate::hittable::{
    hittable::HitRecord, hittable::Hittable, hittable_list::HittableList, sphere::Sphere,
};

// image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 5;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::new();
    let col = Color::new(1.0, 1.0, 1.0);

    // if we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        // simulate a ray bouncing off an object
        let target: Point3d = rec.p + rec.normal + Point3d::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_dir: Point3d = r.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    return ((1.0 - t) * col) + (t * Color::new(0.5, 0.7, 1.0));
}

fn main() {
    // world
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));

    // camera
    let cam = Camera::new();

    // render
    println!("P3\n{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);
    // send out rays from center of camera into world
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            // sample around pixel to color to antialias
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + random_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v: f32 = (j as f32 + random_f32()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color);
        }
    }
    eprintln!("\nDone.");
}
