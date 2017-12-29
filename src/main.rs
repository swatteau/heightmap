extern crate diamondsquare;

use diamondsquare::TerrainGenerator;

fn main() {
    let mut generator = TerrainGenerator::new(5);
    generator.generate(1.0, 10.0, 20.0, 40.0);
    println!("{:#?}", generator.get_terrain());
}
