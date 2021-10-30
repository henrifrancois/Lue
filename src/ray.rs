use crate::{Vec3, Point3, Color};

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
        return Point3::new_vec(self.origin.into_inner() + self.direction * t)
    }

    pub fn ray_color(&self) -> Color {
        let unit_direction = self.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Color::new_vec(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t);
    }
}