use rand::random_range;

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

    pub fn columnar_chunks(height: usize, width: usize, count: usize, offset: i32) -> Vec<Chunk> {
        let chunk_size = width / count;
        let offset = random_range(0..offset) as usize;

        let mut xmin = 0;
        let mut xmax = chunk_size - offset;
        let mut chunks = Vec::new();
        while xmin < width {
            chunks.push(Self::build(xmin, xmax, 0, height));
            xmin = xmax;
            xmax = (xmax + chunk_size).min(width);
        }

        chunks
    }
}
