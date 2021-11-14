use crate::vec3::{Vec3, Point3, Color, Hit, Object};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction * t
    }

    fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - *center;
        let a = self.direction().vlength_squared();
        let hb = oc.dot(self.direction());
        let c = oc.vlength_squared() - radius.powf(2.0);
        let discriminant = hb * hb - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-hb - discriminant.sqrt()) / a;
        }
    }
    

    pub fn ray_color(&self, world: &impl Object) -> Color {
        let mut rec = Hit { point: Point3::zero(), normal: Point3::zero(), t: 0.0, front_face: false };
        if world.hit( &self, (0.0, std::f64::INFINITY), &mut rec) {
            return (Color::new(1.0,1.0,1.0) + rec.normal) * 0.5;
        }
        let unit_direction = self.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    }
}