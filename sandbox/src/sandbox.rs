use rand::random_range;

use crate::chunks::Chunk;
use crate::helpers::random_coprime;
use crate::particle_params::ParticleParams;
use crate::particle_updates::Update;
use crate::particles::Particle;
use crate::particles::ParticleType;
use crate::thread_ptr::RawPtrMut;

pub struct Handler {
    pub x: usize,
    pub y: usize,
    pub here: Particle,
    pub sandbox: RawPtrMut<SandBox>,
}

impl Handler {
    pub fn build(x: usize, y: usize, mut sandbox: RawPtrMut<SandBox>) -> Self {
        Handler { x, y, here: sandbox.deref().get(x, y), sandbox }
    }

    pub fn update(&mut self) {
        if let Some(behavior) = self.here.behavior {
            behavior.update(self);
        }
    }

    pub fn get(&mut self, dx: isize, dy: isize) -> Particle {
        let (nx, ny) = self.relative_index(dx, dy);
        self.sandbox.deref().get(nx, ny)
    }

    pub fn get_mut(&mut self, dx: isize, dy: isize) -> Option<&mut Particle> {
        let (nx, ny) = self.relative_index(dx, dy);
        self.sandbox.deref().get_mut(nx, ny)
    }

    pub fn get_mut_unchecked(&mut self, dx: isize, dy: isize) -> &mut Particle {
        let (nx, ny) = self.relative_index(dx, dy);
        self.sandbox.deref().get_mut_unchecked(nx, ny)
    }

    pub fn get_mut_here(&mut self) -> &mut Particle {
        self.sandbox.deref().get_mut_unchecked(self.x, self.y)
    }

    pub fn get_params(&mut self, dx: isize, dy: isize) -> ParticleParams {
        let particle = self.get(dx, dy);
        self.sandbox.deref().particleparams[particle.species as usize]
    }

    pub fn get_params_here(&mut self) -> ParticleParams {
        self.sandbox.deref().particleparams[self.here.species as usize]
    }

    pub fn swap(&mut self, tx: isize, ty: isize) {
        let (nx, ny) = self.relative_index(tx, ty);
        let from = self.sandbox.deref().index(self.x, self.y);
        let to = self.sandbox.deref().index(nx, ny);
        self.sandbox.deref().swap(from, to);
        (self.x, self.y) = (nx, ny);
        self.here = self.sandbox.deref().get(self.x, self.y);
    }

    fn relative_index(&self, dx: isize, dy: isize) -> (usize, usize) {
        let nx = (self.x as isize + dx) as usize;
        let ny = (self.y as isize + dy) as usize;
        (nx, ny)
    }
}

pub struct SandBox {
    pub height: usize,
    pub width: usize,
    pub grid: Vec<Particle>,
    pub particleparams: [ParticleParams; ParticleType::EnumLength as usize],
    pub thread_count: usize,
    pub cluster_size: usize,
    pub chunk_offset: i32,
    pub flipflop: isize,
    pub tick: u32,
    pub color_freq: u32,
    pub color_shift: u32,
}

impl SandBox {
    pub fn build(width: usize, height: usize) -> Self {
        SandBox {
            height,
            width,
            grid: (0..width * height).map(|_| Particle::build(ParticleType::Empty)).collect(),
            particleparams: ParticleParams::base_params_builder(),
            thread_count: usize::default(),
            cluster_size: usize::default(),
            chunk_offset: i32::default(),
            flipflop: 1,
            tick: u32::default(),
            color_freq: u32::default(),
            color_shift: u32::default(),
        }
    }

    pub fn update_par(&mut self) {
        use std::thread::spawn;

        let chunks = Chunk::columnar_chunks(self.height, self.width, self.thread_count, self.chunk_offset);
        let selfptr = RawPtrMut::build(self as *mut SandBox);

        // two pass processing to mitigate (never can eliminate) data races at
        // the chunk borders
        let mut handles = Vec::new();
        (0..chunks.len()).step_by(2).map(|idx| chunks[idx]).for_each(|chunk| {
            handles.push(spawn(move || {
                Self::process_zig_zag(chunk, selfptr);
            }));
        });
        // first pass join
        handles.into_iter().for_each(|handle| {
            handle.join().unwrap();
        });

        let mut handles = Vec::new();
        (0..chunks.len()).skip(1).step_by(2).map(|idx| chunks[idx]).for_each(|chunk| {
            handles.push(spawn(move || {
                Self::process_zig_zag(chunk, selfptr);
            }));
        });
        // second pass join - testing and this conserves >> 99.9999%
        // of mass; less than one in a million particle is being raced and
        // overwritten
        handles.into_iter().for_each(|handle| {
            handle.join().unwrap();
        });
        self.flipflop = -self.flipflop;
        self.tick += 1;
        self.color_shift = self.tick / self.color_freq;
    }

    pub fn to_color(&self) -> Vec<u32> {
        self.grid.iter().map(|ele| ele.color).collect()
    }

    #[allow(dead_code)]
    pub fn to_debug(&self) -> Vec<u32> {
        self.grid
            .iter()
            .map(|ele| {
                if ele.awake {
                    0xff00ffff
                }
                else {
                    0xff000000
                }
            })
            .collect()
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if !self.inbounds(x, y) {
            return Particle::build(ParticleType::OutOfBounds);
        }
        let index = self.index(x, y);
        {
            debug_assert!(index < self.width * self.height);
        }
        self.grid[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        if !self.inbounds(x, y) {
            return None;
        }
        let index = self.index(x, y);
        {
            debug_assert!(index < self.width * self.height);
        }
        Some(&mut self.grid[index])
    }

    pub fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Particle {
        let index = self.index(x, y);
        {
            debug_assert!(index < self.width * self.height);
        }
        &mut self.grid[index]
    }

    pub fn add_particle(&mut self, species: ParticleType, x: usize, y: usize) {
        if !self.inbounds(x, y) {
            return;
        }
        let index = self.index(x, y);
        {
            debug_assert!(index < self.width * self.height);
        }
        if self.grid[index].is_empty() || species == ParticleType::Empty {
            self.grid[index] = Particle::build_color(species, self.color_shift);
        }
    }

    pub fn add_cluster(&mut self, species: ParticleType, x: usize, y: usize) {
        let bounds = (self.cluster_size / 2) as isize;
        (-bounds..=bounds).for_each(|dy| {
            (-bounds..=bounds).for_each(|dx| {
                if dx * dx + dy * dy <= bounds * bounds {
                    self.add_particle(species, x.saturating_add_signed(dx), y.saturating_add_signed(dy));
                }
            });
        });
    }

    pub fn clear(&mut self) {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let index = self.index(x, y);
                {
                    debug_assert!(index < self.width * self.height);
                }
                self.grid[index] = Particle::build_color(ParticleType::Empty, self.color_shift);
            });
        });
    }

    fn swap(&mut self, from: usize, to: usize) {
        {
            debug_assert!(from < self.width * self.height && to < self.width * self.height);
        }
        (self.grid[from], self.grid[to]) = (self.grid[to], self.grid[from])
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// there are 3 methods for processing a chunk each with varying speeds
    /// and behaviors two are pseudo-random and one is a true random iteration
    /// basically, the speed ordering is exactly what you would expect and the
    /// "pretty" ordering is also exactly what you would expect. zig-zag is by far
    /// the fastest but looks very obviously strictly sequential
    #[allow(dead_code)]
    fn process_pcg(chunk: Chunk, ptr: RawPtrMut<SandBox>) {
        // algorithm is known as pcg rangom pcg-random.org
        // weird random rectangular region coprime iteration from physics stack exchange
        let width = chunk.xmax - chunk.xmin;
        let height = chunk.ymax - chunk.ymin;
        let area = width * height;
        let offset = random_range(0..area);
        let step = random_coprime(area);
        for index in 0..area {
            let linear_index = (offset + step * index) % area;

            let x = (linear_index % width) + chunk.xmin;
            let y = (linear_index / width) + chunk.ymin;

            let mut handler = Handler::build(x, y, ptr);
            handler.update();
        }
    }

    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn process_zig_zag(chunk: Chunk, ptr: RawPtrMut<SandBox>) {
        // process bottom to top, skipping rows and zig-zagging on x
        (chunk.ymin..chunk.ymax).rev().step_by(2).for_each(|y| {
            (chunk.xmin..chunk.xmax).for_each(|x| {
                let mut handler = Handler::build(x, y, ptr);
                handler.update();
            });
        });
        (chunk.ymin..chunk.ymax).rev().skip(1).step_by(2).for_each(|y| {
            (chunk.xmin..chunk.xmax).rev().for_each(|x| {
                let mut handler = Handler::build(x, y, ptr);
                handler.update();
            });
        });

        return;

        // bottom to top and zig-zagging on x
        (chunk.ymin..chunk.ymax).rev().for_each(|y| {
            if y % 2 == 0 {
                (chunk.xmin..chunk.xmax).for_each(|x| {
                    let mut handler = Handler::build(x, y, ptr);
                    handler.update();
                });
            }
            else {
                (chunk.xmin..chunk.xmax).rev().for_each(|x| {
                    let mut handler = Handler::build(x, y, ptr);
                    handler.update();
                });
            }
        });

        panic!("stagger algorithms are mutually exclusive");
    }

    #[allow(dead_code)]
    fn process_true_random(chunk: Chunk, ptr: RawPtrMut<SandBox>) {
        use rand::rng;
        use rand::seq::SliceRandom;

        // collects every index first and entirely randomizes the iteration
        let mut rng = rng();
        let mut indices: Vec<(usize, usize)> =
            (chunk.ymin..chunk.ymax).flat_map(|y| (chunk.xmin..chunk.xmax).map(move |x| (x, y))).collect();
        indices.shuffle(&mut rng);
        indices.iter().for_each(|&(x, y)| {
            let mut handler = Handler::build(x, y, ptr);
            handler.update();
        });
    }
}
