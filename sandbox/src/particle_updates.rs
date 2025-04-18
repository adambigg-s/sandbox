use rand::random_bool;
use rand::random_range;

use crate::helpers::LineTracer;
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

impl Update for FreeFall {
    fn update(&self, handler: &mut Handler) {
        let direc = if handler.here.direction_bias {
            handler.sandbox.deref().flipflop
        }
        else {
            -handler.sandbox.deref().flipflop
        };
        let params = handler.get_params_here();
        handler.get_mut_here().vy = (handler.get_mut_here().vy + params.gravity)
            .clamp(-params.terminal_velocity, params.terminal_velocity);
        handler.reup_here();

        let x0 = handler.x as isize;
        let y0 = handler.y as isize;
        let mut linetrace = LineTracer::build(x0, y0, handler.here.vx, handler.here.vy);
        let mut moved = false;
        while let Some((nx, ny)) = linetrace.step() {
            let dx = nx - x0;
            let dy = ny - y0;

            if dx == 0 && dy == 0 {
                continue;
            }

            if handler.get(dx, dy).is_empty() {
                handler.swap(dx, dy);
                moved = true;
            }
            else if handler.get(dx, dy).is_falling() {
                handler.get_mut_unchecked(dx, dy).vx = handler.here.vx;
                handler.get_mut_unchecked(dx, dy).vy = handler.here.vy;
                moved = true;
            }
            else if handler.here.vy.abs() > params.speed_to_bounce {
                handler.get_mut_here().vx =
                    handler.here.vy * direc as f32 * params.horizontal_transfer * random_range(0.0..1.0);
                handler.get_mut_here().vy = params.minimal_velocity;
            }
        }
        if !moved {
            handler.get_mut_here().vx = 0.;
            handler.get_mut_here().vy = params.minimal_velocity;
            handler.get_mut_here().stop_falling();
        }
    }
}

impl Update for Solid {
    fn update(&self, handler: &mut Handler) {
        if !handler.here.is_awake() {
            if handler.get(0, 1).is_empty()
                || handler.get(0, 1).is_liquid()
                || handler.get(0, 1).is_gas()
                || handler.get(0, 1).is_falling()
            {
                handler.get_mut_here().awake = true;
                handler.get_mut_here().begin_falling();
            }
            else {
                return;
            }
        }

        let direc = if handler.here.direction_bias {
            handler.sandbox.deref().flipflop
        }
        else {
            -handler.sandbox.deref().flipflop
        };
        let params = handler.get_params_here();
        let mut moved = false;

        if handler.get(0, 1).is_empty() && !handler.get(0, 1).is_solid() {
            handler.swap(0, 1);
            handler.get_mut_here().begin_falling();
            moved = true;
        }
        else if handler.get(direc, 1).is_empty()
            || handler.get(direc, 1).is_liquid()
            || handler.get(direc, 1).is_gas()
        {
            handler.swap(direc, 1);
            moved = true;
        }
        else if handler.get(-direc, 1).is_empty()
            || handler.get(-direc, 1).is_liquid()
            || handler.get(-direc, 1).is_gas()
        {
            handler.swap(-direc, 1);
            moved = true;
        }
        else if random_bool(params.resistance) {
            handler.get_mut_here().awake = false;
        }

        if moved {
            if let Some(p) = handler.get_mut(1, 0) {
                p.awake = true;
            }
            if let Some(p) = handler.get_mut(-1, 0) {
                p.awake = true;
            }
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
        let params = handler.get_params_here();
        let mut moved = false;

        if handler.get(0, 1).is_empty() {
            handler.get_mut_here().begin_falling();
        }

        if handler.get(0, 1).is_empty()
            || handler.get(0, 1).is_gas()
            || (handler.get(0, 1).is_liquid() && handler.get_params(0, 1).density < params.density)
        {
            handler.swap(0, 1);
            return;
        }
        else if handler.get(direc, 1).is_empty()
            || handler.get(direc, 1).is_gas()
            || (handler.get(direc, 1).is_liquid() && handler.get_params(direc, 1).density < params.density)
        {
            handler.swap(direc, 1);
            return;
        }

        loop {
            if (handler.get(direc, 0).is_empty()
                || (handler.get(direc, 0).is_liquid()
                    && handler.get(direc, 0).species != handler.here.species))
                && (handler.get(-direc, 1).is_liquid() || random_bool(params.viscosity))
            {
                handler.swap(direc, 0);
                moved = true;
                continue;
            }
            break;
        }

        if !moved && handler.get(0, -1).is_empty() && random_bool(params.fluid_shimmer) {
            handler.swap(0, -1);
        }
    }
}

impl Update for Gas {
    fn update(&self, handler: &mut Handler) {
        let direc = handler.sandbox.deref().flipflop;
        let params = handler.get_params_here();

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
