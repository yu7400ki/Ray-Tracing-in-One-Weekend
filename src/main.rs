mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use image::RgbImage;

use crate::utils::*;
use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::*;
use crate::camera::Camera;

fn ray_color<T>(r: Ray, world: &T) -> Color
where
    T: Hittable,
{
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0));
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

    let mut world = HittableList::new();
    world.add(Sphere::new(point3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(point3(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new();

    for j in (0..image_height).rev() {
        let y = image_height - j - 1;
        print!("\rScan lines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = color(0.0, 0.0, 0.0);
            let x = i;
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            let pixel = pixel_color.to_rgb(samples_per_pixel);
            image.put_pixel(x, y, pixel);
        }
    }

    image.save(path).unwrap();
    println!("\nDone.");
}
