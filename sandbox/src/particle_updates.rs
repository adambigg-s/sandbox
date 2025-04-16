use rand::random_bool;
use rand::random_range;

use crate::particles::Particle;
use crate::particles::ParticleType;
use crate::sandbox::Handler;

pub trait Update {
    fn update(&self, handler: &mut Handler);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Solid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Liquid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gas;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeFall;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Behavior {
    Solid(Solid),
    Liquid(Liquid),
    Gas(Gas),
    FreeFall(FreeFall),
}

impl Update for Behavior {
    fn update(&self, handler: &mut Handler) {
        match self {
            Self::Solid(solid) => solid.update(handler),
            Self::Liquid(liquid) => liquid.update(handler),
            Self::Gas(gas) => gas.update(handler),
            Self::FreeFall(freefall) => freefall.update(handler),
        }
    }
}

// don't look at this yet, barely works and needs to be refactored just haven't
// gotten to it
impl Update for FreeFall {
    fn update(&self, h: &mut Handler) {
        let par = h.sandbox.deref().particleparams[h.here.species as usize];
        let particle = h.get_mut_here();
        particle.vy += 0.01;
        particle.vx += random_range(-0.1..0.1);
        particle.vy = particle.vy.max(1.);
        particle.vy = particle.vy.min(par.max_fallspeed);

        let vx = particle.vx;
        let vy = particle.vy;

        let steps = f32::max(vx.abs(), vy.abs()) as usize;

        if steps == 0 {
            return;
        }

        let dx = vx / steps as f32;
        let dy = vy / steps as f32;

        let mut fx = 0.0;
        let mut fy = 0.0;

        for _ in 0..steps {
            fx += dx;
            fy += dy;

            let ix = fx.round() as isize;
            let iy = fy.round() as isize;

            if ix == 0 && iy == 0 {
                continue;
            }

            if h.get(ix, iy).is_empty() {
                h.swap(ix, iy);
            }
            else {
                h.get_mut_here().stop_falling();
                return;
            }
        }
    }
}

impl Update for Solid {
    fn update(&self, h: &mut Handler) {
        if !h.here.is_awake() {
            if h.get(0, 1).is_empty() || h.get(0, 1).is_liquid() || h.get(0, 1).is_gas() {
                h.get_mut_here().awake = true;
                h.get_mut_here().freefall();
            }
            else {
                return;
            }
        }

        let d = if h.here.direction_bias {
            h.sandbox.deref().flipflop
        }
        else {
            -h.sandbox.deref().flipflop
        };
        let par = h.sandbox.deref().particleparams[h.here.species as usize];
        let mut mo = false;

        if h.get(0, 1).is_empty() || h.get(0, 1).is_liquid() || h.get(0, 1).is_gas() {
            h.swap(0, 1);
            h.get_mut_here().freefall();
            mo = true;
        }
        else if h.get(d, 1).is_empty() || h.get(d, 1).is_liquid() || h.get(d, 1).is_gas() {
            h.swap(d, 1);
            mo = true;
        }
        else if h.get(-d, 1).is_empty() || h.get(-d, 1).is_liquid() || h.get(-d, 1).is_gas() {
            h.swap(-d, 1);
            mo = true;
        }
        else if random_bool(par.resistance) {
            h.get_mut_here().awake = false;
        }

        if mo {
            if let Some(p) = h.get_mut(1, 0) {
                p.awake = true;
            }
            if let Some(p) = h.get_mut(-1, 0) {
                p.awake = true;
            }
        }
    }
}

impl Update for Liquid {
    fn update(&self, h: &mut Handler) {
        let d = if h.here.direction_bias {
            h.sandbox.deref().flipflop
        }
        else {
            -h.sandbox.deref().flipflop
        };
        let par = h.sandbox.deref().particleparams[h.here.species as usize];

        if h.get(0, 1).is_empty() {
            h.swap(0, 1);
            return;
        }
        for dx in [d, -d] {
            if h.get(dx, 1).is_empty() || h.get(dx, 1).is_gas() {
                h.swap(dx, 1);
                return;
            }
        }
        let mut moved = false;
        for dist in 1..=par.spread_velocity {
            for dx in [dist as isize * d, -(dist as isize) * d] {
                if h.get(dx, 0).is_empty() {
                    h.swap(dx, 0);
                    moved = true;
                    break;
                }
            }
            if moved {
                break;
            }
        }
    }
}

impl Update for Gas {
    fn update(&self, handler: &mut Handler) {
        let direc = handler.sandbox.deref().flipflop;
        let params = handler.sandbox.deref().particleparams[handler.here.species as usize];

        if random_bool(params.volatility) {
            *handler.get_mut_here() = Particle::build(ParticleType::Empty);
        }
        if handler.get(0, -1).is_empty() && random_bool(params.vertical_affinity) {
            handler.swap(0, -1);
        }
        else if handler.get(direc, 0).is_empty() && random_bool(params.horizontal_affinity) {
            handler.swap(direc, 0);
        }
    }
}
