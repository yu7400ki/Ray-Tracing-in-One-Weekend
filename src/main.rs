mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::Camera;
use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
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
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return color(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(color(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = point3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range_double(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(point3(0.0, 1.0, 0.0), 1.0, material1));
    let material2 = Lambertian::new(color(0.4, 0.2, 0.1));
    world.add(Sphere::new(point3(-4.0, 1.0, 0.0), 1.0, material2));
    let material3 = Metal::new(color(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(point3(4.0, 1.0, 0.0), 1.0, material3));

    world
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

    let world = random_scene();

    let lookfrom = point3(13.0, 2.0, 3.0);
    let lookat = point3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

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
