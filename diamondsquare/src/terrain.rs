use super::{Position, Random};

#[derive(Debug)]
pub struct Terrain {
    size: usize,
    values: Vec<f64>,
}

impl Terrain {
    pub fn new(size: usize) -> Terrain {
        Terrain {
            size: size,
            values: vec![0f64; size * size],
        }
    }

    pub fn get(&self, p: Position) -> f64 {
        self.values[self.index(p)]
    }

    pub fn set(&mut self, p: Position, height: f64) {
        let index = self.index(p);
        self.values[index] = height;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn index(&self, p: Position) -> usize {
        self.size * p.1 + p.0
    }
}

pub struct TerrainGenerator<'a> {
    order: u32,
    rand: Option<&'a mut Random>,
    terrain: Terrain,
}

fn size(order: u32) -> usize {
    2usize.pow(order) + 1
}

impl<'a> TerrainGenerator<'a> {
    pub fn new(order: u32) -> TerrainGenerator<'a> {
        TerrainGenerator {
            order: order,
            rand: None,
            terrain: Terrain::new(size(order)),
        }
    }

    pub fn generate(&mut self, h1: f64, h2: f64, h3: f64, h4: f64) {
        let s = size(self.order) - 1;
        self.terrain.set(Position(0, 0), h1);
        self.terrain.set(Position(0, s), h2);
        self.terrain.set(Position(s, 0), h3);
        self.terrain.set(Position(s, s), h4);
        for i in 0..self.order {
            self.step(i + 1)
        }
    }

    pub fn get_terrain(&self) -> &Terrain {
        &self.terrain
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
        let average_part = {
            let Position(col, row) = p;
            let mut value = 0f64;
            value += self.terrain.get(Position(col - unit, row - unit));
            value += self.terrain.get(Position(col - unit, row + unit));
            value += self.terrain.get(Position(col + unit, row - unit));
            value += self.terrain.get(Position(col + unit, row + unit));
            value / 4.0
        };

        let height = average_part + self.get_random_part(p, unit);
        self.terrain.set(p, height);
    }

    fn diamond_step(&mut self, p: Position, unit: usize) {
        let average_part = {
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

        let height = average_part + self.get_random_part(p, unit);
        self.terrain.set(p, height);
    }

    fn get_random_part(&mut self, p: Position, unit: usize) -> f64 {
        match self.rand {
            Some(ref mut rand) => rand.random(p, unit),
            None => 0.0,
        }
    }
}