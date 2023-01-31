mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::Camera;
use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::ray::*;
use crate::sphere::Sphere;
use crate::utils::*;
use crate::vec3::*;

fn ray_color<T>(r: Ray, world: &T, depth: u32) -> Color
where
    T: Hittable,
{
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(ray(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).unwrap();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut image = RgbImage::new(image_width, image_height);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();
    world.add(Sphere::new(point3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(point3(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new();

    let bar = ProgressBar::new(image_height as u64);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {percent}% ({eta})")
            .unwrap(),
    );

    for j in (0..image_height).rev() {
        bar.inc(1);
        let y = image_height - j - 1;
        for i in 0..image_width {
            let mut pixel_color = color(0.0, 0.0, 0.0);
            let x = i;
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            let pixel = pixel_color.to_rgb(samples_per_pixel);
            image.put_pixel(x, y, pixel);
        }
    }

    image.save(path).unwrap();
    bar.finish();
    println!("Done. Output saved to {}", path)
}
