extern crate diamondsquare;

use diamondsquare::{Position, ExtrinsicFn, TerrainGenerator};

struct MyRandom;

impl ExtrinsicFn for MyRandom {
    fn evaluate(&mut self, p: Position, unit: usize) -> f64 {
        0.0
    }
}

fn main() {
    let mut generator = TerrainGenerator::new(5, Box::new(MyRandom));
    generator.generate(1.0, 10.0, 20.0, 40.0);
    println!("{:#?}", generator.get_terrain());
}
