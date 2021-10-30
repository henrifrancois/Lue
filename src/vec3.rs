use std::ops::{Add, Sub, Mul, Div, Neg, Index};

#[derive(Clone, Copy)]
pub struct Vec3 {
    inner: [f64; 3],
}

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

    pub fn dot(&self, other: Self) -> f64 {
        self.inner[0] * other.inner[0] + self.inner[1] + other.inner[1] + self.inner[2] + other.inner[2]
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

#[derive(Copy, Clone)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color(Vec3::new(x,y,z))
    }

    pub fn new_vec(v: Vec3) -> Self {
        Color(v)
    }

    pub fn into_inner(self) -> Vec3 {
        self.0
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }

    pub fn write_color(&self) {
        println!("{} {} {}", 
        255.999 * self.x(),
        255.999 * self.y(),
        255.999 * self.z())
    }
}
#[derive(Copy, Clone)]
pub struct Point3(Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x,y,z))
    }

    pub fn new_vec(v: Vec3) -> Self {
        Point3(v)
    }

    pub fn into_inner(self) -> Vec3 {
        self.0
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }
    
}