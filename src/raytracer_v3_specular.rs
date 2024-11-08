//! ... + specular reflections

use crate::common::{self, *};
use image::{ImageBuffer, Rgb};

/// Compute lighting intensity at a point.
fn compute_lighting(point: &Vec3, normal: &Vec3, view: &Vec3, specular: Option<f64>, lights: &[Light]) -> f64 {
    lights.iter().fold(0.0, |intensity, light| {
        intensity + match &light.light_type {
            LightType::Ambient => light.intensity,
            LightType::Point { position } => {
                let direction = position.sub(point);
                let diffuse = calculate_diffuse_intensity(normal, &direction, light.intensity);
                let specular = calculate_specular_intensity(normal, &direction, view, specular, light.intensity);
                diffuse + specular
            },
            LightType::Directional { direction } => {
                let diffuse = calculate_diffuse_intensity(normal, direction, light.intensity);
                let specular = calculate_specular_intensity(normal, direction, view, specular, light.intensity);
                diffuse + specular
            }
        }
    })
}

/// Calculate the diffuse lighting intensity based on the normal and light direction
fn calculate_diffuse_intensity(normal: &Vec3, direction: &Vec3, intensity: f64) -> f64 {
    let n_dot_l = normal.dot(direction);
    if n_dot_l <= 0.0 {
        return 0.0;
    }
    intensity * n_dot_l / (normal.length() * direction.length())
}

/// Calculate the specular lighting intensity
fn calculate_specular_intensity(normal: &Vec3, light_dir: &Vec3, view: &Vec3, specular: Option<f64>, intensity: f64) -> f64 {
    if let Some(s) = specular {
        let n_dot_l = normal.dot(light_dir);
        if n_dot_l > 0.0 {
            // R = 2 * N * dot(N, L) - L
            let reflection = normal.scale(2.0 * n_dot_l).sub(light_dir);
            let r_dot_v = reflection.dot(view);
            
            if r_dot_v > 0.0 {
                return intensity * (r_dot_v / (reflection.length() * view.length())).powf(s);
            }
        }
    }
    0.0
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
            let lighting_intensity = compute_lighting(
                &point,
                &normal,
                &direction.scale(-1.0),  // View direction (opposite of ray direction)
                sphere.material.specular,
                lights
            );
            sphere.material.color.scale(lighting_intensity)
        })
        .unwrap_or(Color::new(255, 255, 255)) // white background
}

/// Main function to run the ray tracer with specular reflections.
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

    img.save("img/specular.png").unwrap();
}