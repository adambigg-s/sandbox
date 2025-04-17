use rand::random_bool;

use crate::helpers::LinearInterpolator;
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
        let params = h.get_params_here();
        h.get_mut_here().vy =
            (h.get_mut_here().vy + params.gravity).clamp(params.minimal_velocity, params.terminal_velocity);
        let mut interpolater = LinearInterpolator::build(h.x, h.y, h.get_mut_here().vx, h.get_mut_here().vy);

        let x0 = h.x as isize;
        let y0 = h.y as isize;
        while let Some((nx, ny)) = interpolater.next() {
            let dx = nx - x0;
            let dy = ny - y0;

            if dx == 0 && dy == 0 {
                continue;
            }

            if h.get(dx, dy).is_empty() {
                h.swap(dx, dy);
            }
            else if h.get(dx, dy).is_falling() {
                h.get_mut_unchecked(dx, dy).vx = h.get_mut_here().vx;
                h.get_mut_unchecked(dx, dy).vy = h.get_mut_here().vy;
            }
            else {
                h.get_mut_here().vy = params.minimal_velocity;
                h.get_mut_here().stop_falling();
            }
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
