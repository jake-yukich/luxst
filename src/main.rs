/*  starting with basic ray tracing
    - we initially assume a fixewd viewing position, camera position O (0,0,0)
    - the camera orientation is also fixed, looking down the positive z axis
        - the positive x-axis extends to the right of the camera
        - the positive y-axis extends up from the camera
    - we assume the frame (camera view) has fixed dimensions V_w and V_h
        - the frame is distance d from the camera, centered on the z axis
    - for simplicity, V_w = V_h = d = 1
    - canvas coordinates of a given pixel: C_x, C_y
    - V_x = C_x * V_w / C_w
    - V_y = C_y * V_h / C_h
    - V_z = d
    - rays pass through O, so any point P in the ray can be expressed as P = O + t(V - O), where t is any real number
    - we'll add some simple spheres to the scene
        - spheres are defined by their center c and radius r
        - for a point P on the surface of a sphere, the distance from c to P is equal to r
        - we can express this as (P - c) dot (P - c) = r^2
    
    here is what we need to implement, in pseudocode:
        origin = (0,0,0)
        for x = -C_w/2 to C_w/2
            for y = -C_h/2 to C_h/2
                D = CanvasToViewport(x, y)
                color = trace(origin, D, 1, inf)
                canvas.PutPixel(x, y, color)
        
        function CanvasToViewport(x, y):
            return (x * V_w / C_w, y * V_h / C_h, d)

        function trace(origin, D, t_min, t_max):
            closest_t = inf
            closest_sphere = None
            for sphere in spheres:
                t1, t2 = intersect_sphere(origin, D, sphere)
                if t1 >= t_min and t1 <= t_max and t1 < closest_t:
                    closest_t = t1
                    closest_sphere = sphere
                if t2 >= t_min and t2 <= t_max:
                    closest_t = t2
                    closest_sphere = sphere
            if closest_sphere is None:
                return background_color
            else:
                return closest_sphere.color

        function intersect_sphere(origin, D, sphere):
            r = sphere.radius
            co = origin - sphere.center

            a = dot(D, D)
            b = 2 * dot(co, D)
            c = dot(co, co) - r**2

            discriminant = b**2 - 4*a*c
            if discriminant < 0:
                return inf, inf
            
            t1 = (-b + discriminant**0.5) / (2*a)
            t2 = (-b - discriminant**0.5) / (2*a)
            return t1, t2

        viewport_size = 1 x 1
        projection_plane_d = 1
        sphere {
            center = (0, -1, 3)
            radius = 1
            color = (255, 0, 0)
        }
        sphere {
            center = (2, 0, 4)
            radius = 1
            color = (0, 0, 255)
        }
        sphere {
            center = (-2, 0, 4)
            radius = 1
            color = (0, 255, 0)
        }
*/

use std::f64;

// Define a 3D vector structure
#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    // Dot product of two vectors
    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Subtract two vectors
    fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Define a color structure
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Define a sphere structure
struct Sphere {
    center: Vec3,
    radius: f64,
    color: Color,
}

// Constants for viewport and canvas
const VIEWPORT_SIZE: f64 = 1.0;
const PROJECTION_PLANE_D: f64 = 1.0;
const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = 600;

// Function to convert canvas coordinates to viewport coordinates
fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        x as f64 * VIEWPORT_SIZE / CANVAS_WIDTH as f64,
        -y as f64 * VIEWPORT_SIZE / CANVAS_HEIGHT as f64, // Note the negation here
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

    let discriminant = b * b - 4.0 * a * c;
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

fn main() {
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
    ];

    // Create a simple PPM image
    println!("P3");
    println!("{} {}", CANVAS_WIDTH, CANVAS_HEIGHT);
    println!("255");

    let origin = Vec3::new(0.0, 0.0, 0.0);
    for y in 0..CANVAS_HEIGHT {
        for x in 0..CANVAS_WIDTH {
            let direction = canvas_to_viewport(
                x as i32 - (CANVAS_WIDTH as i32 / 2),
                y as i32 - (CANVAS_HEIGHT as i32 / 2)
            );
            let color = trace(&origin, &direction, 1.0, f64::INFINITY, &spheres);
            println!("{} {} {}", color.r, color.g, color.b);
        }
    }
}