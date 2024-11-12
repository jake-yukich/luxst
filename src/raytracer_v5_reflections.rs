//! ... + reflections

use crate::common::{self, *};
use image::{ImageBuffer, Rgb};

/// Compute lighting intensity at a point.
///
/// TODO:
/// * fix redundant shadow code with a cleaner refactor
fn compute_lighting(point: &Vec3, normal: &Vec3, view: &Vec3, specular: Option<f64>, lights: &[Light], spheres: &[Sphere]) -> f64 {
    lights.iter().fold(0.0, |intensity, light| {
        intensity + match &light.light_type {
            LightType::Ambient => light.intensity,
            LightType::Point { position } => {
                let direction = position.sub(point);

                // check for shadow
                let (shadow_sphere, _) = closest_intersection(point, &direction, 0.001, direction.length(), spheres);
                if shadow_sphere.is_some() {
                    0.0
                } else {
                    let diffuse = calculate_diffuse_intensity(normal, &direction, light.intensity);
                    let specular = calculate_specular_intensity(normal, &direction, view, specular, light.intensity);
                    diffuse + specular
                }
            },
            LightType::Directional { direction } => {
                // check for shadow
                let (shadow_sphere, _) = closest_intersection(point, direction, 0.001, f64::INFINITY, spheres);
                if shadow_sphere.is_some() {
                    0.0
                } else {
                    let diffuse = calculate_diffuse_intensity(normal, direction, light.intensity);
                    let specular = calculate_specular_intensity(normal, direction, view, specular, light.intensity);
                    diffuse + specular
                }
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

/// Reflect a ray off a surface
fn reflect_ray(normal: &Vec3, ray: &Vec3) -> Vec3 {
    normal.scale(2.0 * normal.dot(ray)).sub(ray)
}

/// Trace a ray through the scene and compute the color at the intersection point.
fn trace(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &[Sphere], lights: &[Light], recursion_depth: u32) -> Color {
    let (closest_sphere, closest_t) = closest_intersection(origin, direction, t_min, t_max, spheres);

    closest_sphere
        .map(|sphere| {
            let point = origin.add(&direction.scale(closest_t));
            let normal = point.sub(&sphere.center).normalize();
            
            // Calculate local color
            let local_color = {
                let lighting_intensity = compute_lighting(
                    &point,
                    &normal,
                    &direction.scale(-1.0),
                    sphere.material.specular,
                    lights,
                    spheres
                );
                sphere.material.color.scale(lighting_intensity)
            };

            let r = sphere.material.reflective.unwrap_or(0.0);
            
            if recursion_depth <= 0 || r <= 0.0 {
                local_color
            } else {
                let reflected_ray = reflect_ray(&normal, &direction.scale(-1.0));
                
                // Recursive call
                let reflected_color = trace(
                    &point,
                    &reflected_ray,
                    0.001,
                    f64::INFINITY,
                    spheres,
                    lights,
                    recursion_depth - 1
                );

                // Blend local and reflected colors based on reflectivity
                local_color.scale(1.0 - r).add(&reflected_color.scale(r))
            }
        })
        .unwrap_or(Color::new(0, 0, 0)) // black background
}

/// Find the closest intersection between a ray and all spheres in the scene.
fn closest_intersection<'a>(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &'a [Sphere]) -> (Option<&'a Sphere>, f64) {
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

    (closest_sphere, closest_t)
}

/// Main function to run the ray tracer with reflections.
pub fn main() {
    let scene = common::scene::Scene::basic_scene();
    let mut img = ImageBuffer::new(common::config::CANVAS_WIDTH, common::config::CANVAS_HEIGHT);
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let direction = common::geometry::canvas_to_viewport(
            x as i32 - (common::config::CANVAS_WIDTH as i32 / 2),
            y as i32 - (common::config::CANVAS_HEIGHT as i32 / 2)
        );
        let color = trace(&origin, &direction, 1.0, f64::INFINITY, &scene.spheres, &scene.lights, 3);
        *pixel = Rgb([color.r, color.g, color.b]);
    }

    img.save("img/reflections.png").unwrap();
}
