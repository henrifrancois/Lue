mod vec3;
use vec3::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);
    for i in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", i);
        for j in 0..image_width {
            let pixel_color = Color::new(Vec3::new(i as f64 / (image_width as f64 - 1.0), j as f64 / (image_height as f64 - 1.0), 0.25_f64));
            pixel_color.write_color();
        }
    }
    eprint!("\nDone.");
}
