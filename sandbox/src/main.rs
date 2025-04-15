mod chunks;
mod helpers;
mod particle_updates;
mod particles;
mod sandbox;
mod thread_ptr;

use helpers::get_inputs;
use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

use sandbox::SandBox;

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    let mut window =
        Window::new("sandbox", WIDTH, HEIGHT, WindowOptions { scale: Scale::X2, ..Default::default() })
            .expect("failed to grab window handle");
    let mut world = SandBox::build(WIDTH, HEIGHT);
    world.thread_count = 20;
    world.cluster_size = 30;
    world.chunk_offset = 20;

    while window.is_open() {
        let time = std::time::Instant::now();

        get_inputs(&window, &mut world);
        world.update_par();
        window
            .update_with_buffer(&world.to_color(), world.width, world.height)
            .expect("failed to update window");

        println!("fps: {:.2}", 1. / time.elapsed().as_secs_f32());
        println!("particles: {}", world.particle_count);
        println!("actual: {}", world.grid.iter().filter(|ele| !ele.species.is_empty()).count());
        println!("threads: {}", world.thread_count);
    }
}
