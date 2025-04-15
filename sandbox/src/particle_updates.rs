use rand::random_bool;

use crate::particles::Gas;
use crate::particles::Liquid;
use crate::particles::Particle;
use crate::particles::ParticleType;
use crate::particles::Solid;
use crate::particles::Update;
use crate::sandbox::Handler;

impl Update for Solid {
    fn update(&self, ha: &mut Handler) {
        let direc = if ha.here.direction_bias {
            ha.sandbox.deref().flipflop
        }
        else {
            -ha.sandbox.deref().flipflop
        };
        if ha.get(0, 1).species.is_empty() || ha.get(0, 1).species.is_water() {
            ha.swap(0, 1);
        }
        else if ha.get(direc, 1).species.is_empty() {
            ha.swap(direc, 1);
        }
        else if ha.get(-direc, 1).species.is_empty() {
            ha.swap(-direc, 1);
        }
    }
}

impl Update for Liquid {
    fn update(&self, handler: &mut Handler) {
        let direc = if handler.here.direction_bias {
            handler.sandbox.deref().flipflop
        }
        else {
            -handler.sandbox.deref().flipflop
        };
        if handler.get(0, 1).species.is_empty() {
            handler.swap(0, 1);
        }
        else if handler.get(direc, 1).species.is_empty() {
            handler.swap(direc, 1);
        }
        else if handler.get(-direc, 1).species.is_empty() {
            handler.swap(-direc, 1);
        }
        else {
            for _ in 0..handler.sandbox.deref().particleparams[ParticleType::Water as usize].spread_velocity {
                if handler.get(direc, 0).species.is_empty() || handler.get(direc, 0).species.is_water() {
                    handler.swap(direc, 0);
                    continue;
                }
                break;
            }
        }
    }
}

impl Update for Gas {
    fn update(&self, ha: &mut Handler) {
        if random_bool(0.01) {
            *ha.get_mut_here() = Particle::build(ParticleType::Empty);
        }
        let direc = if ha.here.direction_bias {
            ha.sandbox.deref().flipflop
        }
        else {
            -ha.sandbox.deref().flipflop
        };
        if ha.get(0, -1).species.is_empty() {
            ha.swap(0, -1);
        }
        else if ha.get(direc, -1).species.is_empty() {
            ha.swap(direc, -1);
        }
        else if ha.get(-direc, -1).species.is_empty() {
            ha.swap(-direc, -1);
        }
    }
}
