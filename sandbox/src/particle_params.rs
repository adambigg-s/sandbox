use crate::particles::ParticleType;

#[derive(Default, Clone, Copy)]
pub struct ParticleParams {
    pub spread_velocity: usize,
    pub max_fallspeed: f32,
    pub resistance: f64,
    pub volatility: f64,
    pub vertical_affinity: f64,
    pub horizontal_affinity: f64,
}

impl ParticleParams {
    pub fn base_params_builder() -> [Self; ParticleType::EnumLength as usize] {
        let mut params = [Self::default(); ParticleType::EnumLength as usize];
        params[ParticleType::Sand as usize] = sand_params();
        params[ParticleType::Water as usize] = water_params();
        params[ParticleType::Smoke as usize] = smoke_params();
        params[ParticleType::Gravel as usize] = gravel_params();
        params
    }
}

fn sand_params() -> ParticleParams {
    ParticleParams { resistance: 0.05, max_fallspeed: 3., ..Default::default() }
}

fn water_params() -> ParticleParams {
    ParticleParams { spread_velocity: 50, ..Default::default() }
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
    ParticleParams { resistance: 0.5, max_fallspeed: 3., ..Default::default() }
}
