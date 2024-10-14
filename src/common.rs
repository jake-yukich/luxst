// Common structures and functions

use std::f64;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Define a sphere structure
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
}