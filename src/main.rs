extern crate diamondsquare;

use diamondsquare::{TerrainGenerator};
use diamondsquare::extrinsic;


fn main() {
    let mut generator = TerrainGenerator::new(5, Box::new(extrinsic::Null));
    generator.set_corners(1.0, 10.0, 20.0, 40.0);
    let mut terrain = generator.generate();
    println!("{:#?}", terrain);
    println!("{:?}", terrain.extents());

    terrain.rescale(0.0, 255.0);
    println!("{:?}", terrain.extents());
}
