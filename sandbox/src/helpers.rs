use std::time::Duration;

use minifb::Key;
use minifb::MouseButton;
use minifb::MouseMode;
use minifb::Window;
use rand::random_range;

use crate::particles::ParticleType;
use crate::sandbox::SandBox;

pub struct LinearInterpolator {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub steps: usize,
    pub curr: usize,
}

impl LinearInterpolator {
    pub fn build(x: usize, y: usize, dx: f32, dy: f32) -> Self {
        let steps = (dx.abs()).max(dy.abs()).ceil() as usize;
        LinearInterpolator {
            x: x as f32,
            y: y as f32,
            vx: dx / steps as f32,
            vy: dy / steps as f32,
            steps,
            curr: 0,
        }
    }

    pub fn next(&mut self) -> Option<(isize, isize)> {
        if self.curr >= self.steps {
            return None;
        }

        self.curr += 1;

        self.x += self.vx;
        self.y += self.vy;

        Some((self.x.round() as isize, self.y.round() as isize))
    }
}

pub fn get_inputs(window: &mut Window, world: &mut SandBox) {
    let (mx, my) = window.get_mouse_pos(MouseMode::Clamp).unwrap();
    if window.get_mouse_down(MouseButton::Left) {
        world.add_particle(ParticleType::Sand, mx as usize, my as usize);
    }
    if window.get_mouse_down(MouseButton::Right) {
        world.add_particle(ParticleType::Water, mx as usize, my as usize);
    }
    if window.is_key_down(Key::I) {
        println!("particle here: {:?}", world.get(mx as usize, my as usize));
    }
    if window.is_key_down(Key::C) {
        world.add_cluster(ParticleType::Sand, mx as usize, my as usize);
    }
    if window.is_key_down(Key::W) {
        world.add_cluster(ParticleType::Water, mx as usize, my as usize);
    }
    if window.is_key_down(Key::S) {
        world.add_cluster(ParticleType::Stone, mx as usize, my as usize);
    }
    if window.is_key_down(Key::Key3) {
        world.add_cluster(ParticleType::Smoke, mx as usize, my as usize);
    }
    if window.is_key_down(Key::G) {
        world.add_cluster(ParticleType::Empty, mx as usize, my as usize);
    }
    if window.is_key_down(Key::A) {
        world.add_cluster(ParticleType::Gravel, mx as usize, my as usize);
    }
    if window.is_key_down(Key::D) {
        world.add_cluster(ParticleType::Wood, mx as usize, my as usize);
    }
    if window.is_key_down(Key::O) {
        world.add_cluster(ParticleType::Oil, mx as usize, my as usize);
    }
    if window.is_key_down(Key::R) {
        world.clear();
    }
    if window.is_key_down(Key::P) {
        window.update_with_buffer(&world.to_debug(), world.width, world.height).expect("window update");
        std::thread::sleep(Duration::from_millis(100));
    }
    if window.is_key_down(Key::Equal) {
        world.thread_count += 1;
        std::thread::sleep(Duration::from_millis(100));
    }
    if window.is_key_down(Key::Minus) {
        world.thread_count -= 1;
        std::thread::sleep(Duration::from_millis(100));
    }
    if window.is_key_down(Key::Key1) {
        world.cluster_size -= 1;
    }
    if window.is_key_down(Key::Key2) {
        world.cluster_size += 1;
    }
}

pub fn color_near(red: u8, green: u8, blue: u8, randvar: u32, timevar: u32, time: u32) -> u32 {
    use rand::random;

    let offset = |base: u8| {
        let delta = (random::<i8>() % (randvar as i8)) as i16;
        let angle = 2. * 3.14199 * (time as f32 / timevar as f32);
        let gamma = (angle.sin() * (randvar as f32 / 2.)).round() as i16;
        (base as i16 + delta + gamma).clamp(0, 255) as u8
    };

    (0xff << 24) | ((offset(red) as u32) << 16) | ((offset(green) as u32) << 8) | (offset(blue) as u32)
}

pub fn greatest_common_divisor(mut rhs: usize, mut lhs: usize) -> usize {
    while lhs != 0 {
        let temp = lhs;
        lhs = rhs % lhs;
        rhs = temp;
    }

    rhs
}

pub fn random_coprime(target: usize) -> usize {
    loop {
        let candidate = random_range(1..target);
        if greatest_common_divisor(candidate, target) == 1 {
            return candidate;
        }
    }
}
