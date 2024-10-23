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

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn scale(&self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        self.scale(1.0 / length)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn scale(&self, t: f64) -> Color {
        Color::new(
            (self.r as f64 * t).min(255.0) as u8,
            (self.g as f64 * t).min(255.0) as u8,
            (self.b as f64 * t).min(255.0) as u8,
        )
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
}

pub fn generate_default_spheres() -> Vec<Sphere> {
    // Define spheres in the scene
    let spheres = vec![
        Sphere {
            center: Vec3::new(0.0, -1.0, 3.0),
            radius: 1.0,
            color: Color { r: 255, g: 0, b: 0 },
        },
        Sphere {
            center: Vec3::new(2.0, 0.0, 4.0),
            radius: 1.0,
            color: Color { r: 0, g: 0, b: 255 },
        },
        Sphere {
            center: Vec3::new(-2.0, 0.0, 4.0),
            radius: 1.0,
            color: Color { r: 0, g: 255, b: 0 },
        },
        Sphere {
            center: Vec3::new(0.0, -5001.0, 0.0),
            radius: 5000.0,
            color: Color { r: 255, g: 255, b: 0 },
        },
    ];

    spheres
}

pub enum LightType {
    Ambient,
    Directional { direction: Vec3 },
    Point { position: Vec3 },
}

pub struct Light {
    pub light_type: LightType,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn new_ambient(intensity: f64, color: Color) -> Self {
        Light {
            light_type: LightType::Ambient,
            intensity,
            color,
        }
    }

    pub fn new_directional(direction: Vec3, intensity: f64, color: Color) -> Self {
        Light {
            light_type: LightType::Directional { direction },
            intensity,
            color,
        }
    }

    pub fn new_point(position: Vec3, intensity: f64, color: Color) -> Self {
        Light {
            light_type: LightType::Point { position },
            intensity,
            color,
        }
    }
}
