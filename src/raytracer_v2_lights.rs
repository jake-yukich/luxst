//! Basic ray tracing + lights + diffuse reflections

use crate::common::{self, *};
use image::{ImageBuffer, Rgb};

/// Compute lighting intensity at a point.
fn compute_lighting(point: &Vec3, normal: &Vec3, lights: &[Light]) -> f64 {
    lights.iter().fold(0.0, |intensity, light| {
        intensity + match &light.light_type {
            LightType::Ambient => light.intensity,
            LightType::Point { position } | LightType::Directional { direction: position } => {
                let direction = if let LightType::Point { position } = &light.light_type {
                    position.sub(point).normalize()
                } else {
                    position.normalize()
                };
                let n_dot_l = normal.dot(&direction);
                if n_dot_l > 0.0 {
                    light.intensity * n_dot_l / (normal.length().powi(2))
                } else {
                    0.0
                }
            },
        }
    })
}

/// Trace a ray through the scene and compute the color at the intersection point.
fn trace(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &[Sphere], lights: &[Light]) -> Color {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in spheres {
        let (t1, t2) = common::geometry::intersect_ray_sphere(origin, direction, sphere);
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
        .map(|sphere| {
            let point = origin.add(&direction.scale(closest_t));
            let normal = point.sub(&sphere.center).normalize();
            let lighting_intensity = compute_lighting(&point, &normal, lights);
            sphere.material.color.scale(lighting_intensity)
        })
        .unwrap_or(Color::new(255, 255, 255)) // white background
}

/// Entry point for ray tracing with lights and diffuse reflections.
pub fn main() {
    let scene = common::scene::Scene::basic_scene();
    let mut img = ImageBuffer::new(common::config::CANVAS_WIDTH, common::config::CANVAS_HEIGHT);
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let direction = common::geometry::canvas_to_viewport(
            x as i32 - (common::config::CANVAS_WIDTH as i32 / 2),
            y as i32 - (common::config::CANVAS_HEIGHT as i32 / 2)
        );
        let color = trace(&origin, &direction, 1.0, f64::INFINITY, &scene.spheres, &scene.lights);
        *pixel = Rgb([color.r, color.g, color.b]);
    }

    img.save("img/lights.png").unwrap();
}