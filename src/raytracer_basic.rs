/*
basic ray tracing
*/

use crate::common::*;
use image::{ImageBuffer, Rgb};

// Constants for viewport and canvas
const VIEWPORT_SIZE: f64 = 1.0;
const PROJECTION_PLANE_D: f64 = 1.0;
const CANVAS_WIDTH: u32 = 400;
const CANVAS_HEIGHT: u32 = 400;

// Function to convert canvas coordinates to viewport coordinates
fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        x as f64 * VIEWPORT_SIZE / CANVAS_WIDTH as f64,
        -y as f64 * VIEWPORT_SIZE / CANVAS_HEIGHT as f64,
        PROJECTION_PLANE_D,
    )
}

// Function to intersect a ray with a sphere
fn intersect_sphere(origin: &Vec3, direction: &Vec3, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = origin.sub(&sphere.center);

    let a = direction.dot(direction);
    let b = 2.0 * co.dot(direction);
    let c = co.dot(&co) - r * r;

    let discriminant: f64 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}

// Function to trace a ray and find the color of the intersected object
fn trace<'a>(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &'a [Sphere]) -> &'a Color {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in spheres {
        let (t1, t2) = intersect_sphere(origin, direction, sphere);
        if t1 >= t_min && t1 <= t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if t2 >= t_min && t2 <= t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    closest_sphere
        .map(|sphere| &sphere.color)
        .unwrap_or(&Color { r: 255, g: 255, b: 255 }) // Background color (white)
}

pub fn main() {
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

    // Create an image buffer
    let mut img = ImageBuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let direction = canvas_to_viewport(
            x as i32 - (CANVAS_WIDTH as i32 / 2),
            y as i32 - (CANVAS_HEIGHT as i32 / 2)
        );
        let color = trace(&origin, &direction, 1.0, f64::INFINITY, &spheres);
        *pixel = Rgb([color.r, color.g, color.b]);
    }

    // Save image
    img.save("basic.png").unwrap();
}