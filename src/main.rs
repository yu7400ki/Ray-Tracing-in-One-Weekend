mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use image::RgbImage;

use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::*;

fn ray_color<T>(r: Ray, world: &T) -> Color
where
    T: Hittable,
{
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

    let mut world = HittableList::new();
    world.add(Sphere::new(point3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(point3(0.0, -100.5, -1.0), 100.0));

    for j in (0..image_height).rev() {
        let y = image_height - j - 1;
        print!("\rScan lines remaining: {} ", j);
        for i in 0..image_width {
            let x = i;
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r, &world);

            let pixel = pixel_color.to_rgb();
            image.put_pixel(x, y, pixel);
        }
    }

    image.save(path).unwrap();
    println!("\nDone.");
}
