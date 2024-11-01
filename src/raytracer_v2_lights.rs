// /*
// basic ray tracing + lights + diffuse reflections
// */

// use crate::common::{self, *};
// use image::{ImageBuffer, Rgb};

// // Constants for viewport and canvas
// const VIEWPORT_SIZE: f64 = 1.0;
// const PROJECTION_PLANE_D: f64 = 1.0;
// const CANVAS_WIDTH: u32 = 400;
// const CANVAS_HEIGHT: u32 = 400;

// // Function to convert canvas coordinates to viewport coordinates
// fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
//     Vec3::new(
//         x as f64 * VIEWPORT_SIZE / CANVAS_WIDTH as f64,
//         -y as f64 * VIEWPORT_SIZE / CANVAS_HEIGHT as f64,
//         PROJECTION_PLANE_D,
//     )
// }

// // Function to intersect a ray with a sphere
// fn intersect_sphere(origin: &Vec3, direction: &Vec3, sphere: &Sphere) -> (f64, f64) {
//     let r = sphere.radius;
//     let co = origin.sub(&sphere.center);

//     let a = direction.dot(direction);
//     let b = 2.0 * co.dot(direction);
//     let c = co.dot(&co) - r * r;

//     let discriminant: f64 = b * b - 4.0 * a * c;
//     if discriminant < 0.0 {
//         (f64::INFINITY, f64::INFINITY)
//     } else {
//         let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
//         let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
//         (t1, t2)
//     }
// }

// // Function to trace a ray and find the color of the intersected object
// fn trace(origin: &Vec3, direction: &Vec3, t_min: f64, t_max: f64, spheres: &[Sphere], lights: &[Light]) -> Color {
//     let mut closest_t = f64::INFINITY;
//     let mut closest_sphere = None;

//     for sphere in spheres {
//         let (t1, t2) = intersect_sphere(origin, direction, sphere);
//         if t1 >= t_min && t1 <= t_max && t1 < closest_t {
//             closest_t = t1;
//             closest_sphere = Some(sphere);
//         }
//         if t2 >= t_min && t2 <= t_max && t2 < closest_t {
//             closest_t = t2;
//             closest_sphere = Some(sphere);
//         }
//     }

//     closest_sphere
//         .map(|sphere| {
//             let point = origin.add(&direction.scale(closest_t));
//             let mut normal = point.sub(&sphere.center);
//             normal = normal.normalize();
            
//             let lighting_intensity = compute_lighting(&point, &normal, lights);
//             sphere.color.scale(lighting_intensity)
//         })
//         .unwrap_or(Color { r: 255, g: 255, b: 255 })
// }

// fn compute_lighting(point: &Vec3, normal: &Vec3, lights: &[Light]) -> f64 {
//     /* 
//     Computes the lighting intensity at a point in 3D space by summing contributions from all light sources.

//     The function handles three types of lights:
//     - Ambient: Provides constant base illumination
//     - Point: Light emanating from a specific position
//     - Directional: Light coming from a specific direction

//     For point and directional lights, the intensity is modulated by the cosine of the angle
//     between the surface normal and light direction (n·l = cos(θ)), implementing Lambert's 
//     cosine law for diffuse reflection. This means surfaces facing directly toward the light
//     (θ = 0°) receive maximum illumination, while surfaces at grazing angles (θ approaching 90°)
//     receive less light.

//     Parameters:
//         point: The 3D point being illuminated
//         normal: The surface normal vector at that point
//         lights: Slice containing all light sources in the scene
        
//     Returns:
//         A scalar intensity value in [0, inf) that can be used to scale the surface color.
//         Values > 1 indicate additive lighting from multiple sources.
//     */
//     lights.iter().fold(0.0, |intensity, light| {
//         intensity + match &light.light_type {
//             LightType::Ambient => light.intensity,
//             LightType::Point { position } | LightType::Directional { direction: position } => {
//                 let direction = if let LightType::Point { position } = &light.light_type {
//                     position.sub(point).normalize()
//                 } else {
//                     position.normalize()
//                 };
//                 let n_dot_l = normal.dot(&direction);
//                 if n_dot_l > 0.0 {
//                     light.intensity * n_dot_l / (normal.length().powi(2))
//                 } else {
//                     0.0
//                 }
//             },
//         }
//     })
// }

// pub fn main() {
//     // Generate default spheres
//     let spheres = common::generate_default_spheres();

//     let lights = vec![
//         Light::new_ambient(0.2, Color::new(255, 255, 255)),
//         Light::new_directional(Vec3::new(1.0, 4.0, 4.0), 0.2, Color::new(255, 255, 255)),
//         Light::new_point(Vec3::new(2.0, 1.0, 0.0), 0.6, Color::new(255, 255, 255)),
//     ];

//     let mut img = ImageBuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
//     let origin = Vec3::new(0.0, 0.0, 0.0);
//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let direction = canvas_to_viewport(
//             x as i32 - (CANVAS_WIDTH as i32 / 2),
//             y as i32 - (CANVAS_HEIGHT as i32 / 2)
//         );
//         let color = trace(&origin, &direction, 1.0, f64::INFINITY, &spheres, &lights);
//         *pixel = Rgb([color.r, color.g, color.b]);
//     }

//     // Save image
//     img.save("lights.png").unwrap();
// }