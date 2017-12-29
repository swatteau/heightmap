extern crate diamondsquare;

use diamondsquare::{TerrainGenerator};
use diamondsquare::extrinsic;

fn main() {
    let mut generator = TerrainGenerator::new(5, Box::new(extrinsic::Null));
    generator.set_corners(1.0, 10.0, 20.0, 40.0);
    generator.generate();
    println!("{:#?}", generator.get_terrain());
}
