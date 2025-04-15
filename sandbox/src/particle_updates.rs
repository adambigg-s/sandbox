use rand::random_bool;
use rand::random_iter;

use crate::particles::Gas;
use crate::particles::Liquid;
use crate::particles::Particle;
use crate::particles::ParticleType;
use crate::particles::Solid;
use crate::particles::Update;
use crate::sandbox::Handler;

impl Update for Solid {
    fn update(&self, handler: &mut Handler) {
        if !handler.here.awake {
            if handler.get(0, 1).species.is_empty() {
                handler.get_mut_here().awake = true;
            }
            return;
        }

        let direc = if handler.here.direction_bias {
            handler.sandbox.deref().flipflop
        }
        else {
            -handler.sandbox.deref().flipflop
        };
        let params = handler.sandbox.deref().particleparams[handler.here.species as usize];

        if handler.get(0, 1).species.is_empty() {
            handler.swap(0, 1);
        }
        else if random_bool(params.resistance) {
            handler.get_mut_here().awake = false;
        }
        else if handler.get(direc, 1).species.is_empty() {
            handler.swap(direc, 1);
        }
        else if handler.get(-direc, 1).species.is_empty() {
            handler.swap(-direc, 1);
        }
        handler.get_mut(1, 0).awake = true;
        handler.get_mut(-1, 0).awake = true;
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
        let params = handler.sandbox.deref().particleparams[handler.here.species as usize];

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
            for _ in 0..params.spread_velocity {
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
    fn update(&self, handler: &mut Handler) {
        if random_bool(0.01) {
            *handler.get_mut_here() = Particle::build(ParticleType::Empty);
        }
        let direc = if handler.here.direction_bias {
            handler.sandbox.deref().flipflop
        }
        else {
            -handler.sandbox.deref().flipflop
        };
        if handler.get(0, -1).species.is_empty() {
            handler.swap(0, -1);
        }
        else if handler.get(direc, -1).species.is_empty() {
            handler.swap(direc, -1);
        }
        else if handler.get(-direc, -1).species.is_empty() {
            handler.swap(-direc, -1);
        }
    }
}
