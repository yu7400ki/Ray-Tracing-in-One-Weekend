mod vec3;
mod ray;

use crate::vec3::{Color, Vec3};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color: Color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            let (ir, ig, ib) = pixel_color.to_rgb();
            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
