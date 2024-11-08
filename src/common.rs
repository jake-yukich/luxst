//! Core data structures and utilities for ray tracing.

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

#[derive(Clone, Copy)]
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

pub struct Material {
    pub color: Color,
    pub specular: Option<f64>, // e.g. 500 for shiny, -1 for matte
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

pub fn generate_default_spheres() -> Vec<Sphere> {
    vec![
        Sphere {
            center: Vec3::new(0.0, -1.0, 3.0),
            radius: 1.0,
            material: Material {
                color: Color::new(255, 0, 0), // red
                specular: Some(500.0), // shiny
            },
        },
        Sphere {
            center: Vec3::new(2.0, 0.0, 4.0),
            radius: 1.0,
            material: Material {
                color: Color::new(0, 0, 255), // blue
                specular: Some(500.0), // shiny
            },
        },
        Sphere {
            center: Vec3::new(-2.0, 0.0, 4.0),
            radius: 1.0,
            material: Material {
                color: Color::new(0, 255, 0), // green
                specular: Some(10.0), // a bit shiny
            },
        },
        Sphere {
            center: Vec3::new(0.0, -5001.0, 0.0),
            radius: 5000.0,
            material: Material {
                color: Color::new(255, 255, 0), // yellow
                specular: Some(1000.0), // very shiny
            },
        },
    ]
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

pub fn generate_default_lights() -> Vec<Light> {
    vec![
        Light::new_ambient(0.2, Color::new(255, 255, 255)),
        Light::new_directional(Vec3::new(1.0, 4.0, 4.0), 0.2, Color::new(255, 255, 255)),
        Light::new_point(Vec3::new(2.0, 1.0, 0.0), 0.6, Color::new(255, 255, 255)),
    ]
}

pub mod config {
    pub const VIEWPORT_SIZE: f64 = 1.0;
    pub const PROJECTION_PLANE_D: f64 = 1.0;
    pub const CANVAS_WIDTH: u32 = 400;
    pub const CANVAS_HEIGHT: u32 = 400;
}

pub mod geometry {
    use super::*;

    /// Convert canvas coordinates to viewport coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the canvas (image buffer), assumed to be centered
    /// * `y` - The y-coordinate of the canvas (image buffer), assumed to be centered
    ///
    /// # Returns
    ///
    /// Returns the viewport coordinates, with z = distance to projection plane (viewport).
    pub fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f64 * config::VIEWPORT_SIZE / config::CANVAS_WIDTH as f64,
            -y as f64 * config::VIEWPORT_SIZE / config::CANVAS_HEIGHT as f64,
            config::PROJECTION_PLANE_D,
        )
    }

    pub fn intersect_ray_sphere(origin: &Vec3, direction: &Vec3, sphere: &Sphere) -> (f64, f64) {
        let r = sphere.radius;
        let co = origin.sub(&sphere.center);

        let a = direction.dot(direction);
        let b = 2.0 * co.dot(direction);
        let c = co.dot(&co) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            (f64::INFINITY, f64::INFINITY)
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            (t1, t2)
        }
    }
}

pub mod scene {
    use super::*;
    
    pub struct Scene {
        pub spheres: Vec<Sphere>,
        pub lights: Vec<Light>,
    }

    impl Scene {
        pub fn basic_scene() -> Self {
            Scene {
                spheres: generate_default_spheres(),
                lights: generate_default_lights(),
            }
        }
    }
}