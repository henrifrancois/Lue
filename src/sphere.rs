use crate::vec3::{Point3, Hit, Object};
use crate::ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, range: (f64, f64), hit_record: &mut Hit) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().vlength_squared();
        let hb = oc.dot(ray.direction());
        let c = oc.vlength_squared() - self.radius.powf(2.0);
        
        let discriminant = hb.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();
        let mut root = (-hb - sqrtd) / a;
        
        if root < range.0 || range.1 < root {
            root = (-hb - sqrtd) / a;
            if root < range.0 || range.1 < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        return true;
    }
}

pub struct SphereList(pub Vec<Rc<dyn Object>>); // my first ever use of a shared, reference-counted pointer in Rust!! 🦀

impl Object for SphereList {
    fn hit(&self, ray: &Ray, range: (f64, f64), hit_record: &mut Hit) -> bool {
        let mut tmp: Hit = Hit { point: Point3::zero(), normal: Point3::zero(), t: 0.0, front_face: false };;
        let mut hit_anything = false;
        let mut closest = range.1;
        for object in &self.0 {
            if object.hit(ray, (range.0, closest), &mut tmp) == true {
                hit_anything = true;
                closest = tmp.t;
                *hit_record = tmp;
            }
        }

        hit_anything
    }
}