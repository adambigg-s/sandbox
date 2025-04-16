use crate::particles::ParticleParams;

pub fn sand_params() -> ParticleParams {
    ParticleParams { resistance: 0.1, max_fallspeed: 4, ..Default::default() }
}

pub fn water_params() -> ParticleParams {
    ParticleParams { spread_velocity: 10, ..Default::default() }
}

pub fn smoke_params() -> ParticleParams {
    ParticleParams {
        volatility: 0.001,
        vertical_affinity: 0.1,
        horizontal_affinity: 0.5,
        ..Default::default()
    }
}
