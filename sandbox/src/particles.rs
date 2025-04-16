use rand::random_bool;

use crate::helpers::color_near;
use crate::particle_updates::Behavior;
use crate::particle_updates::FreeFall;
use crate::particle_updates::Gas;
use crate::particle_updates::Liquid;
use crate::particle_updates::Solid;

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
            behavior: species.base_behavior(),
            color: ParticleType::color(species, 0),
            direction_bias: random_bool(0.5),
            awake: true,
            vx: f32::default(),
            vy: f32::default(),
        }
    }

    pub fn build_color(species: ParticleType, time: u32) -> Self {
        Particle {
            species,
            behavior: species.base_behavior(),
            color: ParticleType::color(species, time),
            direction_bias: random_bool(0.5),
            awake: true,
            vx: f32::default(),
            vy: f32::default(),
        }
    }

    pub fn is_awake(&self) -> bool {
        self.awake
    }

    pub fn is_empty(&self) -> bool {
        self.species == ParticleType::Empty
    }

    pub fn is_falling(&self) -> bool {
        if let Some(behavior) = self.behavior {
            return Behavior::FreeFall(FreeFall) == behavior;
        }
        false
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
    pub fn base_behavior(&self) -> Option<Behavior> {
        match self {
            Self::Sand => Some(Behavior::Solid(Solid)),
            Self::Water => Some(Behavior::Liquid(Liquid)),
            Self::Smoke => Some(Behavior::Gas(Gas)),
            Self::OutOfBounds => Some(Behavior::Solid(Solid)),
            _ => None,
        }
    }

    fn color(self, time: u32) -> u32 {
        match self {
            Self::Empty => color_near(210, 220, 230, 10, 30, time),
            Self::Sand => color_near(255, 200, 130, 25, 30, time),
            Self::Water => color_near(166, 214, 214, 15, 30, time),
            Self::Stone => color_near(190, 190, 200, 30, 30, time),
            Self::Smoke => color_near(30, 30, 30, 25, 30, time),
            Self::OutOfBounds => 0xff00ffff,
            _ => 0xff000000,
        }
    }
}
