//! Basic ray tracing

use crate::common::{self, *};
use image::{ImageBuffer, Rgb};

/// Trace a ray through the scene and return the color of the closest object/intersection.
///
/// # Arguments
///
/// * `origin` - The origin in 3D space, (0, 0, 0), and the assumed camera position.
/// * `direction` - The direction of the ray.
/// * `t_min` - The minimum distance to consider for intersections.
/// * `t_max` - The maximum distance to consider for intersections.
/// * `spheres` - The spheres in the scene.
///
/// # Returns
///
/// Returns the color of the first intersected object, or white if no intersection is found.
fn trace(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &[Sphere]) -> Color {
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
        .map(|sphere| sphere.material.color)
        .unwrap_or(Color::new(255, 255, 255)) // white background
}

/// Entry point for basic ray tracing.
///
/// Creates a scene with default spheres and traces rays through it,
/// saving the output as a PNG image.
pub fn main() {
    let scene = common::scene::Scene::basic_scene();
    let mut img = ImageBuffer::new(common::config::CANVAS_WIDTH, common::config::CANVAS_HEIGHT);
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let direction = common::geometry::canvas_to_viewport(
            x as i32 - (common::config::CANVAS_WIDTH as i32 / 2),
            y as i32 - (common::config::CANVAS_HEIGHT as i32 / 2)
        );
        let color = trace(&origin, &direction, 1.0, f64::INFINITY, &scene.spheres);
        *pixel = Rgb([color.r, color.g, color.b]);
    }

    img.save("img/basic.png").unwrap();
}