use crate::particles::ParticleParams;

pub fn sand_params() -> ParticleParams {
    ParticleParams { resistance: 0.01, max_fallspeed: 4, ..Default::default() }
}

pub fn water_params() -> ParticleParams {
    ParticleParams { spread_velocity: 10, ..Default::default() }
}
