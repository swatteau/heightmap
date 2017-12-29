extern crate diamondsquare;

use diamondsquare::Terrain;

fn main() {
    let mut terrain = Terrain::new(5);
    terrain.generate(1.0, 10.0, 20.0, 40.0);
    println!("{:#?}", terrain);
}
