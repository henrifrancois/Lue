use std::ops::{Add, Sub, Mul, Div, Neg, Index};
use crate::ray::Ray;

#[inline(always)]
fn clamp(x: f64, range: (f64, f64)) -> f64 {
    if x < range.0 {
        return range.0;
    };
    if x > range.1 {
        return range.1;
    };
    return x;
}


#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    inner: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 { inner: [0.0,0.0,0.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { 
            inner: [x,y,z] 
        }
    }

    pub fn x(&self) -> f64 {
        self.inner[0]
    }

    pub fn y(&self) -> f64 {
        self.inner[1]
    }

    pub fn z(&self) -> f64 {
        self.inner[2]
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn vlength(&self) -> f64 {
        return self.vlength_squared().sqrt();
    }

    pub fn vlength_squared(&self) -> f64 {
        return self.inner[0].powf(2.0) + self.inner[1].powf(2.0) + self.inner[2].powf(2.0);
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.inner[0] * other.inner[0] + self.inner[1] * other.inner[1] + self.inner[2] * other.inner[2]
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            inner: [
                self.inner[1] * other.inner[2] - self.inner[2] * other.inner[1],
                self.inner[2] * other.inner[0] - self.inner[0] * other.inner[2],
                self.inner[0] * other.inner[1] - self.inner[1] * other.inner[0]
            ]
        }
    }

    pub fn unit(self) -> Self {
        let len = &self.len();
        return self / (*len as f64);
    } 

    pub fn write_color(&self, samples: u32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();
    
        let scale = 1.0 / (samples as f64);
        r = scale * r;
        g = scale * g;
        b = scale * b;
    
        println!("{} {} {}", 
            (256 * clamp(r, (0.0, 0.999)) as u32),
            (256 * clamp(g, (0.0, 0.999)) as u32),
            (256 * clamp(b, (0.0, 0.999)) as u32)
        );
    }
    
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            inner: [-self.inner[0], -self.inner[1], -self.inner[2]]
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            inner: [self.inner[0] + other.inner[0], self.inner[1] + other.inner[1], self.inner[2] + other.inner[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            inner: [self.inner[0] - other.inner[0], self.inner[1] - other.inner[1], self.inner[2] - other.inner[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            inner: [self.inner[0] * other, self.inner[1] * other, self.inner[2] * other]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            inner: [self.inner[0] / other, self.inner[1] / other, self.inner[2] / other]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.inner[idx]
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

pub trait Object {
    fn hit(&self, ray: &Ray, range: (f64, f64), hit_record: &mut Hit) -> bool;
}

impl Hit {
    #[inline(always)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal =  if self.front_face == true {*outward_normal} else {-(*outward_normal)};
    }
}

