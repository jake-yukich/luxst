/*
run commands, e.g.
   cargo run --features basic
   cargo run --features lights
   cargo run --features reflections
*/

mod common;
mod raytracer_basic;
mod raytracer_v2_lights;

fn main() {
    #[cfg(feature = "basic")]
    {
        raytracer_basic::main();
    }

    #[cfg(feature = "lights")]
    {
        raytracer_v2_lights::main();
    }
}
