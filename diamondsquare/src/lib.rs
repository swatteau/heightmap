#![feature(iterator_step_by)]

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

pub trait Random {
    fn random(&mut self, p: Position, unit: usize) -> f64;
}

pub struct Terrain<'a> {
    order: u32,
    size: usize,
    rand: Option<&'a mut Random>,
    values: Vec<f64>,
}

impl<'a> Terrain<'a> {
    pub fn new(order: u32) -> Terrain<'a> {
        let size = 2usize.pow(order) + 1;
        Terrain {
            order: order,
            size: size,
            rand: None,
            values: vec![0f64; size * size],
        }
    }

    pub fn generate(&mut self, h1: f64, h2: f64, h3: f64, h4: f64) {
        let s = self.size - 1;
        *self.get_mut(Position(0, 0)) = h1;
        *self.get_mut(Position(0, s)) = h2;
        *self.get_mut(Position(s, 0)) = h3;
        *self.get_mut(Position(s, s)) = h4;
        for i in 0..self.order {
            self.step(i + 1)
        }
    }

    fn step(&mut self, rank: u32) {
        let unit = (self.size - 1) / 2usize.pow(rank);

        for row in (unit..self.size - 1).step_by(2 * unit) {
            for col in (unit..self.size - 1).step_by(2 * unit) {
                self.square_step(Position(col, row), unit);
            }
        }

        for row in (0..self.size).step_by(2 * unit) {
            for col in (unit..self.size - 1).step_by(2 * unit) {
                self.diamond_step(Position(col, row), unit);
                self.diamond_step(Position(row, col), unit);
            }
        }
    }

    fn square_step(&mut self, p: Position, unit: usize) {
        let average_part = {
            let Position(col, row) = p;
            let mut value = 0f64;
            value += self.get(Position(col - unit, row - unit));
            value += self.get(Position(col - unit, row + unit));
            value += self.get(Position(col + unit, row - unit));
            value += self.get(Position(col + unit, row + unit));
            value / 4.0
        };

        *self.get_mut(p) = average_part + self.get_random_part(p, unit);
    }

    fn diamond_step(&mut self, p: Position, unit: usize) {
        let average_part = {
            let Position(col, row) = p;
            let (mut neighbors, mut value) = (0, 0f64);
            if col >= unit {
                value += self.get(Position(col - unit, row));
                neighbors += 1;
            }
            if col + unit < self.size {
                value += self.get(Position(col + unit, row));
                neighbors += 1;
            }
            if row >= unit {
                value += self.get(Position(col, row - unit));
                neighbors += 1;
            }
            if row + unit < self.size {
                value += self.get(Position(col, row + unit));
                neighbors += 1;
            }
            value / neighbors as f64
        };

        *self.get_mut(p) = average_part + self.get_random_part(p, unit);
    }

    fn get_random_part(&mut self, p: Position, unit: usize) -> f64 {
        match self.rand {
            Some(ref mut rand) => rand.random(p, unit),
            None => 0.0,
        }
    }

    fn get(&self, p: Position) -> f64 {
        self.values[self.index(p)]
    }

    fn get_mut(&mut self, p: Position) -> &mut f64 {
        let index = self.index(p);
        &mut self.values[index]
    }

    fn index(&self, p: Position) -> usize {
        self.size * p.1 + p.0
    }
}