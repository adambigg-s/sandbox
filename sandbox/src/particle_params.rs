use crate::particles::ParticleType;

#[derive(Default, Clone, Copy)]
pub struct ParticleParams {
    /// when using zig-zag as the update algorithm, particles should never be
    /// permitted to have a terminal velocity of exactly 2. as this interferes
    /// the update order and causes super upgly artifacts
    pub terminal_velocity: f32,
    pub minimal_velocity: f32,
    pub gravity: f32,
    pub resistance: f64,
    pub volatility: f64,
    pub vertical_affinity: f64,
    pub horizontal_affinity: f64,
    pub fluid_shimmer: f64,
    pub viscosity: f64,
    pub density: usize,
    pub speed_to_bounce: f32,
    pub horizontal_transfer: f32,
}

impl ParticleParams {
    pub fn base_params_builder() -> [Self; ParticleType::EnumLength as usize] {
        let mut params = [Self::default(); ParticleType::EnumLength as usize];
        params[ParticleType::Sand as usize] = sand_params();
        params[ParticleType::Water as usize] = water_params();
        params[ParticleType::Smoke as usize] = smoke_params();
        params[ParticleType::Gravel as usize] = gravel_params();
        params[ParticleType::Oil as usize] = oil_params();
        params
    }
}

fn sand_params() -> ParticleParams {
    ParticleParams {
        minimal_velocity: 1.,
        terminal_velocity: 4.,
        gravity: 0.05,
        resistance: 0.2,
        speed_to_bounce: 1.5,
        horizontal_transfer: 0.6,
        ..Default::default()
    }
}

fn water_params() -> ParticleParams {
    ParticleParams {
        minimal_velocity: 1.,
        terminal_velocity: 2.5,
        gravity: 0.05,
        fluid_shimmer: 0.1,
        viscosity: 0.85,
        density: 10,
        speed_to_bounce: 1.5,
        horizontal_transfer: 0.6,
        ..Default::default()
    }
}

fn smoke_params() -> ParticleParams {
    ParticleParams {
        volatility: 0.001,
        vertical_affinity: 0.1,
        horizontal_affinity: 0.5,
        ..Default::default()
    }
}

fn gravel_params() -> ParticleParams {
    ParticleParams {
        minimal_velocity: 1.,
        terminal_velocity: 4.,
        gravity: 0.1,
        resistance: 0.8,
        ..Default::default()
    }
}

fn oil_params() -> ParticleParams {
    ParticleParams {
        minimal_velocity: 1.,
        terminal_velocity: 1.5,
        gravity: 0.01,
        fluid_shimmer: 0.05,
        viscosity: 0.1,
        density: 1,
        speed_to_bounce: 1.5,
        horizontal_transfer: 0.6,
        ..Default::default()
    }
}
