use rand::random_bool;
use rand::random_range;

use crate::chunks::Chunk;
use crate::particles::Particle;
use crate::particles::ParticleParams;
use crate::particles::ParticleType;
use crate::particles::Update;
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
        if let Some(behavior) = self.here.behavior() {
            behavior.update(self);
        }
    }

    pub fn get(&mut self, dx: isize, dy: isize) -> Particle {
        let (nx, ny) = self.relative_index(dx, dy);
        self.sandbox.deref().get(nx, ny)
    }

    pub fn get_mut(&mut self, dx: isize, dy: isize) -> &mut Particle {
        let (nx, ny) = self.relative_index(dx, dy);
        self.sandbox.deref().get_mut(nx, ny)
    }

    pub fn get_mut_here(&mut self) -> &mut Particle {
        self.sandbox.deref().get_mut(self.x, self.y)
    }

    pub fn swap(&mut self, tx: isize, ty: isize) {
        let (nx, ny) = self.relative_index(tx, ty);
        let from = self.sandbox.deref().index(self.x, self.y);
        let to = self.sandbox.deref().index(nx, ny);
        self.sandbox.deref().swap(from, to);
        (self.x, self.y) = (nx, ny);
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
}

impl SandBox {
    pub fn build(width: usize, height: usize) -> Self {
        SandBox {
            height,
            width,
            grid: (0..width * height).map(|_| Particle::build(ParticleType::Empty)).collect(),
            particleparams: ParticleParams::build_for_all(),
            thread_count: usize::default(),
            cluster_size: usize::default(),
            chunk_offset: i32::default(),
            flipflop: 1,
        }
    }

    pub fn update_par(&mut self) {
        use std::thread::spawn;

        let chunks = self.generate_columnar_chunks();
        let selfptr = RawPtrMut::build(self as *mut SandBox);

        // two pass processing to mitigate (never can eliminate) data races at
        // the chunk borders
        let mut handles = Vec::new();
        (0..chunks.len()).step_by(2).map(|idx| chunks[idx]).for_each(|chunk| {
            handles.push(spawn(move || {
                chunk.process_zig_zag(selfptr);
            }));
        });
        // first pass join
        handles.into_iter().for_each(|handle| {
            handle.join().unwrap();
        });

        let mut handles = Vec::new();
        (0..chunks.len()).skip(1).step_by(2).map(|idx| chunks[idx]).for_each(|chunk| {
            handles.push(spawn(move || {
                chunk.process_zig_zag(selfptr);
            }));
        });
        // second pass join - testing and this conserves >> 99.9999%
        // of mass; less than one in a million particle is being raced and
        // overwritten
        handles.into_iter().for_each(|handle| {
            handle.join().unwrap();
        });
        self.flipflop = -self.flipflop;
    }

    pub fn to_color(&self) -> Vec<u32> {
        self.grid.iter().map(|ele| ele.color).collect()
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

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Particle {
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
        if self.grid[index].species.is_empty() || species.is_empty() {
            self.grid[index] = Particle::build(species);
        }
    }

    pub fn add_cluster(&mut self, species: ParticleType, x: usize, y: usize) {
        let bounds = (self.cluster_size / 2) as isize;
        (-bounds..=bounds).for_each(|dy| {
            (-bounds..=bounds).for_each(|dx| {
                if dx * dx + dy * dy <= bounds * bounds && random_bool(0.2) {
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
                self.grid[index] = Particle::build(ParticleType::Empty);
            });
        });
    }

    fn generate_columnar_chunks(&self) -> Vec<Chunk> {
        let chunk_size = self.width / self.thread_count.max(1);
        let offset = random_range(0..self.chunk_offset) as usize;

        let mut xmin = 0;
        let mut xmax = chunk_size - offset;
        let mut chunks = Vec::new();
        while xmin < self.width {
            chunks.push(Chunk::build(xmin, xmax, 0, self.height));
            xmin = xmax;
            xmax = (xmax + chunk_size).min(self.width);
        }

        chunks
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
}
