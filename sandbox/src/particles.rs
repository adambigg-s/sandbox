use rand::random_bool;

use crate::helpers::color_near;
use crate::particle_params::sand_params;
use crate::particle_params::smoke_params;
use crate::particle_params::water_params;
use crate::sandbox::Handler;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Solid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Liquid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gas;

pub trait Update {
    fn update(&self, handler: &mut Handler);
}

#[derive(Default, Clone, Copy)]
pub struct ParticleParams {
    pub spread_velocity: usize,
    pub max_fallspeed: usize,
    pub resistance: f64,
    pub volatility: f64,
    pub vertical_affinity: f64,
    pub horizontal_affinity: f64,
}

impl ParticleParams {
    pub fn build_for_all() -> [Self; ParticleType::EnumLength as usize] {
        let mut params = [Self::default(); ParticleType::EnumLength as usize];
        params[ParticleType::Sand as usize] = sand_params();
        params[ParticleType::Water as usize] = water_params();
        params[ParticleType::Smoke as usize] = smoke_params();
        params
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Behavior {
    Solid(Solid),
    Liquid(Liquid),
    Gas(Gas),
}

impl Update for Behavior {
    fn update(&self, handler: &mut Handler) {
        match self {
            Behavior::Solid(solid) => solid.update(handler),
            Behavior::Liquid(liquid) => liquid.update(handler),
            Behavior::Gas(gas) => gas.update(handler),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParticleType {
    Empty,
    Sand,
    Water,
    Stone,
    Smoke,
    OutOfBounds,
    EnumLength,
}

impl ParticleType {
    pub fn behavior(&self) -> Option<Behavior> {
        match self {
            Self::Sand => Some(Behavior::Solid(Solid)),
            Self::Water => Some(Behavior::Liquid(Liquid)),
            Self::Smoke => Some(Behavior::Gas(Gas)),
            Self::OutOfBounds => Some(Behavior::Solid(Solid)),
            _ => None,
        }
    }

    fn color(self) -> u32 {
        match self {
            Self::Empty => color_near(210, 220, 230, 5),
            Self::Sand => color_near(240, 220, 130, 15),
            Self::Water => color_near(166, 214, 214, 20),
            Self::Stone => color_near(190, 190, 200, 10),
            Self::Smoke => color_near(30, 30, 30, 25),
            Self::OutOfBounds => 0xff00ffff,
            _ => 0xff000000,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub species: ParticleType,
    pub behavior: Option<Behavior>,
    pub color: u32,
    pub direction_bias: bool,
    pub awake: bool,
    pub vx: f32,
    pub vy: f32,
}

impl Particle {
    pub fn build(species: ParticleType) -> Self {
        Particle {
            species,
            behavior: species.behavior(),
            color: ParticleType::color(species),
            direction_bias: random_bool(0.5),
            awake: true,
            vx: f32::default(),
            vy: f32::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.species == ParticleType::Empty
    }

    pub fn is_solid(&self) -> bool {
        if let Some(behavior) = self.behavior {
            return Behavior::Solid(Solid) == behavior;
        }
        false
    }

    pub fn is_liquid(&self) -> bool {
        if let Some(behavior) = self.behavior {
            return Behavior::Liquid(Liquid) == behavior;
        }
        false
    }

    pub fn is_gas(&self) -> bool {
        if let Some(behavior) = self.behavior {
            return Behavior::Gas(Gas) == behavior;
        }
        false
    }
}
