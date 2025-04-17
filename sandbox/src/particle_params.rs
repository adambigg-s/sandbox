use crate::particles::ParticleType;

#[derive(Default, Clone, Copy)]
pub struct ParticleParams {
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
        gravity: 0.1,
        minimal_velocity: 1.,
        resistance: 0.05,
        terminal_velocity: 3.,
        ..Default::default()
    }
}

fn water_params() -> ParticleParams {
    ParticleParams { fluid_shimmer: 0.1, viscosity: 0.7, density: 10, ..Default::default() }
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
    ParticleParams { resistance: 0.5, terminal_velocity: 3., ..Default::default() }
}

fn oil_params() -> ParticleParams {
    ParticleParams { fluid_shimmer: 0.05, viscosity: 0., density: 1, ..Default::default() }
}
