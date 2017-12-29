extern crate diamondsquare;

use diamondsquare::{Position, Random, TerrainGenerator};

struct MyRandom;

impl Random for MyRandom {
    fn random(&mut self, p: Position, unit: usize) -> f64 {
        0.0
    }
}

fn main() {
    let mut rand = MyRandom;
    let mut generator = TerrainGenerator::new(5);
    generator.set_randomizer(&mut rand);
    generator.generate(1.0, 10.0, 20.0, 40.0);
    println!("{:#?}", generator.get_terrain());
}
