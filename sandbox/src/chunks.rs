use rand::random_range;

use crate::helpers::random_coprime;
use crate::sandbox::Handler;
use crate::sandbox::SandBox;
use crate::thread_ptr::RawPtrMut;

/// there are 3 methods for processing a chunk each with varying speeds
/// and behaviors two are pseudo-random and one is a true random iteration
/// basically, the speed ordering is exactly what you would expect and the
/// "pretty" ordering is also exactly what you would expect. zig-zag is by far
/// the fastest but looks very obviously strictly sequential
#[derive(Clone, Copy)]
pub struct Chunk {
    pub xmin: usize,
    pub xmax: usize,
    pub ymin: usize,
    pub ymax: usize,
}

impl Chunk {
    pub fn build(xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> Self {
        Chunk { xmin, xmax, ymin, ymax }
    }

    #[allow(dead_code)]
    pub fn process_pcg(&self, ptr: RawPtrMut<SandBox>) {
        // algorithm is known as pcg rangom pcg-random.org
        // weird random rectangular region coprime iteration from physics stack exchange
        let width = self.xmax - self.xmin;
        let height = self.ymax - self.ymin;
        let area = width * height;
        let offset = random_range(0..area);
        let step = random_coprime(area);
        for index in 0..area {
            let linear_index = (offset + step * index) % area;

            let x = (linear_index % width) + self.xmin;
            let y = (linear_index / width) + self.ymin;

            let mut handler = Handler::build(x, y, ptr);
            handler.update();
        }
    }

    #[allow(dead_code)]
    #[allow(unreachable_code)]
    pub fn process_zig_zag(&self, ptr: RawPtrMut<SandBox>) {
        // process top to bottom, skipping rows and zig-zagging on x
        (self.ymin..self.ymax).step_by(2).for_each(|y| {
            (self.xmin..self.xmax).for_each(|x| {
                let mut handler = Handler::build(x, y, ptr);
                handler.update();
            });
        });
        (self.ymin..self.ymax).skip(1).step_by(2).for_each(|y| {
            (self.xmin..self.xmax).rev().for_each(|x| {
                let mut handler = Handler::build(x, y, ptr);
                handler.update();
            });
        });

        return;

        // bottom to top and zig-zagging on x
        (self.ymin..self.ymax).rev().for_each(|y| {
            if y % 2 == 0 {
                (self.xmin..self.xmax).for_each(|x| {
                    let mut handler = Handler::build(x, y, ptr);
                    handler.update();
                });
            }
            else {
                (self.xmin..self.xmax).rev().for_each(|x| {
                    let mut handler = Handler::build(x, y, ptr);
                    handler.update();
                });
            }
        });

        panic!("stagger algorithms are mutually exclusive");
    }

    #[allow(dead_code)]
    pub fn process_true_random(&self, ptr: RawPtrMut<SandBox>) {
        use rand::rng;
        use rand::seq::SliceRandom;

        // collects every index first and entirely randomizes the iteration
        let mut rng = rng();
        let mut indices: Vec<(usize, usize)> =
            (self.ymin..self.ymax).flat_map(|y| (self.xmin..self.xmax).map(move |x| (x, y))).collect();
        indices.shuffle(&mut rng);
        indices.iter().for_each(|&(x, y)| {
            let mut handler = Handler::build(x, y, ptr);
            handler.update();
        });
    }
}
