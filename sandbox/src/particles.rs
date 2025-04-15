use rand::random_bool;

use crate::helpers::color_near;
use crate::particle_params::sand_params;
use crate::particle_params::water_params;
use crate::sandbox::Handler;

pub struct Solid {}

pub struct Liquid {}

pub struct Gas {}

pub trait Update {
    fn update(&self, handler: &mut Handler);
}

#[derive(Default, Clone, Copy)]
pub struct ParticleParams {
    pub spread_velocity: usize,
    pub max_fallspeed: usize,
    pub resistance: f64,
}

impl ParticleParams {
    pub fn build_for_all() -> [Self; ParticleType::EnumLength as usize] {
        let mut params = [Self::default(); ParticleType::EnumLength as usize];
        params[ParticleType::Sand as usize] = sand_params();
        params[ParticleType::Water as usize] = water_params();
        params
    }
}

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
    pub fn is_empty(self) -> bool {
        self == Self::Empty
    }

    pub fn is_sand(self) -> bool {
        self == Self::Sand
    }

    pub fn is_water(self) -> bool {
        self == Self::Water
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
    pub color: u32,
    pub direction_bias: bool,
    pub awake: bool,
}

impl Particle {
    pub fn build(species: ParticleType) -> Self {
        Particle {
            species,
            color: ParticleType::color(species),
            direction_bias: random_bool(0.5),
            awake: true,
        }
    }

    pub fn behavior(&self) -> Option<Behavior> {
        match self.species {
            ParticleType::Water => Some(Behavior::Liquid(Liquid {})),
            ParticleType::Sand => Some(Behavior::Solid(Solid {})),
            ParticleType::Smoke => Some(Behavior::Gas(Gas {})),
            _ => None,
        }
    }
}
