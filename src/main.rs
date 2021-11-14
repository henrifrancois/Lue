mod vec3;
use vec3::*;

mod ray;
use ray::Ray;

mod sphere;
use sphere::{Sphere, SphereList};

use std::rc::Rc;
use std::f64::consts::PI;

use rand::Rng;

#[inline(always)]
fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;    
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
    
        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World 
    let mut world = SphereList(vec![]); 
    world.0.push(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.0.push(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let cam = Camera::new();


    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_range_f64(0.0, 1.0)) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_range_f64(0.0, 1.0)) / (image_height as f64 - 1.0);
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray.ray_color(&world);
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }
    eprint!("\nDone.");
}
