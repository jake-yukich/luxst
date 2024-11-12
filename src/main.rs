/*
run commands, e.g.
   cargo run --features basic
   cargo run --features lights
   cargo run --features reflections
*/

mod common;
mod raytracer_basic;
mod raytracer_v2_lights;
mod raytracer_v3_specular;
mod raytracer_v4_shadows;
mod raytracer_v5_reflections;

fn main() {
    #[cfg(feature = "basic")]
    {
        raytracer_basic::main();
    }

    #[cfg(feature = "lights")]
    {
        raytracer_v2_lights::main();
    }

    #[cfg(feature = "specular")]
    {
        raytracer_v3_specular::main();
    }

    #[cfg(feature = "shadows")]
    {
        raytracer_v4_shadows::main();
    }

    #[cfg(feature = "reflections")]
    {
        raytracer_v5_reflections::main();
    }
}
