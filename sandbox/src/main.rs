mod chunks;
mod helpers;
mod particle_params;
mod particle_updates;
mod particles;
mod sandbox;
mod thread_ptr;

use helpers::get_inputs;
use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

use sandbox::SandBox;

const WIDTH: usize = 600;
const HEIGHT: usize = 500;
const FPS: usize = 100;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let mut window = Window::new(
        "falling sand sandbox game",
        WIDTH,
        HEIGHT,
        WindowOptions { scale: Scale::X2, ..Default::default() },
    )
    .expect("failed to grab window handle");
    window.set_target_fps(FPS);

    let mut world = SandBox::build(WIDTH, HEIGHT);
    world.thread_count = 20;
    world.cluster_size = 10;
    world.chunk_offset = (WIDTH / world.thread_count) as i32;
    world.color_freq = 2;

    while window.is_open() {
        let time = std::time::Instant::now();

        get_inputs(&mut window, &mut world);
        world.update_par();

        window
            .update_with_buffer(&world.to_color(), world.width, world.height)
            .expect("failed to update window");

        println!("fps: {:.2}", 1. / time.elapsed().as_secs_f32());
        println!("approx threads: {}", world.thread_count);
        println!("chunk offset: {}", world.chunk_offset);
        println!("tick: {}", world.tick / world.color_freq);
    }
}
