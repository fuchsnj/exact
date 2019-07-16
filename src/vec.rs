use crate::exact::Exact;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Vec3 {
    x: Exact,
    y: Exact,
    z: Exact,
}

impl Vec3 {
    pub fn new(x: Exact, y: Exact, z: Exact) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> Exact {
        self.x.clone()
    }
    pub fn y(&self) -> Exact {
        self.y.clone()
    }
    pub fn z(&self) -> Exact {
        self.z.clone()
    }

    pub fn mul_element_wise(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x.clone() * b.x.clone(),
            y: self.y.clone() * b.y.clone(),
            z: self.z.clone() * b.z.clone(),
        }
    }

    pub fn dot(&self, b: &Vec3) -> Exact {
        let c = self.mul_element_wise(&b);
        c.x + c.y + c.z
    }

    pub fn add(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x.clone() + b.x.clone(),
            y: self.y.clone() + b.y.clone(),
            z: self.z.clone() + b.z.clone(),
        }
    }

    pub fn subtract(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x.clone() - b.x.clone(),
            y: self.y.clone() - b.y.clone(),
            z: self.z.clone() - b.z.clone(),
        }
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y.clone() * b.z.clone() - self.z.clone() * b.y.clone(),
            y: self.z.clone() * b.x.clone() - self.x.clone() * b.z.clone(),
            z: self.x.clone() * b.y.clone() - self.y.clone() * b.x.clone(),
        }
    }

    pub fn length(&self) -> Exact {
        let x = self.x.clone() * self.x.clone();
        let y = self.y.clone() * self.y.clone();
        let z = self.z.clone() * self.z.clone();
        let sum = x + y + z;
        sum.sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        Vec3 {
            x: self.x.clone() / length.clone(),
            y: self.y.clone() / length.clone(),
            z: self.z.clone() / length,
        }
    }
}