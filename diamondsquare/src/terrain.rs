use super::{Position, ExtrinsicFn};

#[derive(Debug)]
pub struct Terrain {
    size: usize,
    heights: Vec<f64>,
}

impl Terrain {
    pub fn new(size: usize) -> Terrain {
        Terrain {
            size: size,
            heights: vec![0f64; size * size],
        }
    }

    pub fn get(&self, p: Position) -> f64 {
        self.heights[self.index(p)]
    }

    pub fn set(&mut self, p: Position, height: f64) {
        let index = self.index(p);
        self.heights[index] = height;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn extents(&self) -> (f64, f64) {
        let mut min = self.heights[0];
        let mut max = self.heights[0];
        for h in &self.heights {
            if *h < min {
                min = *h;
            }
            if *h > max {
                max = *h;
            } 
        }
        (min, max)
    }

    pub fn rescale(&mut self, new_min: f64, new_max: f64) {
        let (old_min, old_max) = self.extents();
        let a = (new_max - new_min) / (old_max - old_min);
        let b = new_min - a * old_min;
        for h in self.heights.iter_mut() {
            *h = a * *h + b;
        }
    }

    fn index(&self, p: Position) -> usize {
        self.size * p.1 + p.0
    }
}

pub struct TerrainGenerator {
    order: u32,
    ext_fn: Box<ExtrinsicFn>,
    terrain: Terrain,
}

fn size(order: u32) -> usize {
    2usize.pow(order) + 1
}

impl TerrainGenerator {
    pub fn new(order: u32, ext_fn: Box<ExtrinsicFn>) -> TerrainGenerator {
        TerrainGenerator {
            order: order,
            ext_fn: ext_fn,
            terrain: Terrain::new(size(order)),
        }
    }

    pub fn set_corners(&mut self, h1: f64, h2: f64, h3: f64, h4: f64) {
        let s = size(self.order) - 1;
        self.terrain.set(Position(0, 0), h1);
        self.terrain.set(Position(0, s), h2);
        self.terrain.set(Position(s, 0), h3);
        self.terrain.set(Position(s, s), h4);
    }

    pub fn generate(mut self) -> Terrain {
        for i in 0..self.order {
            self.step(i + 1)
        }
        self.terrain
    }

    fn step(&mut self, rank: u32) {
        let unit = 2usize.pow(self.order - rank);
        let last_index = size(self.order) - 1;

        for row in (unit..last_index).step_by(2 * unit) {
            for col in (unit..last_index).step_by(2 * unit) {
                self.square_step(Position(col, row), unit);
            }
        }

        for row in (0..last_index + 1).step_by(2 * unit) {
            for col in (unit..last_index).step_by(2 * unit) {
                self.diamond_step(Position(col, row), unit);
                self.diamond_step(Position(row, col), unit);
            }
        }
    }

    fn square_step(&mut self, p: Position, unit: usize) {
        let intrinsic_part = {
            let Position(col, row) = p;
            let mut value = 0f64;
            value += self.terrain.get(Position(col - unit, row - unit));
            value += self.terrain.get(Position(col - unit, row + unit));
            value += self.terrain.get(Position(col + unit, row - unit));
            value += self.terrain.get(Position(col + unit, row + unit));
            value / 4.0
        };

        let height = intrinsic_part + self.ext_fn.evaluate(p, unit);
        self.terrain.set(p, height);
    }

    fn diamond_step(&mut self, p: Position, unit: usize) {
        let intrinsic_part = {
            let Position(col, row) = p;
            let (mut neighbors, mut value) = (0, 0f64);
            if col >= unit {
                value += self.terrain.get(Position(col - unit, row));
                neighbors += 1;
            }
            if col + unit < self.terrain.size() {
                value += self.terrain.get(Position(col + unit, row));
                neighbors += 1;
            }
            if row >= unit {
                value += self.terrain.get(Position(col, row - unit));
                neighbors += 1;
            }
            if row + unit < self.terrain.size() {
                value += self.terrain.get(Position(col, row + unit));
                neighbors += 1;
            }
            value / neighbors as f64
        };

        let height = intrinsic_part + self.ext_fn.evaluate(p, unit);
        self.terrain.set(p, height);
    }
}