mod vec3;
use vec3::*;

mod ray;
use ray::Ray;


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.into_inner() - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);
    for i in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", i);
        for j in 0..image_width {
            let u = j as f64 / (image_height as f64 - 1.0);
            let v = i as f64 / (image_width as f64 - 1.0);
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin.into_inner());
            let pixel_color = ray.ray_color();
            pixel_color.write_color();
        }
    }
    eprint!("\nDone.");
}
