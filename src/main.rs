// Copyright 2018 Sébastien Watteau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.use super::{ExtrinsicFn, Position};

extern crate diamondsquare;

use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

use diamondsquare::{Position, Terrain, TerrainGenerator};
use diamondsquare::extrinsic;

fn write_to_file<P: AsRef<Path>>(path: P, terrain: &Terrain) {
    let size = terrain.size();
    let mut bytes: Vec<u8> = Vec::with_capacity(size * size);

    for row in 0..size {
        for col in 0..size {
            bytes.push(terrain.get(Position(col, row)) as u8);
        }
    }

    let file = File::create(path).unwrap();
    let mut buf = BufWriter::new(file);
    buf.write(&bytes).unwrap();
}

fn main() {
    let mut generator = TerrainGenerator::new(10, Box::new(extrinsic::PositionIndependent));
    generator.set_corners(1.0, 10.0, 20.0, 40.0);
    let mut terrain = generator.generate();
    terrain.rescale(0.0, 255.0);
    write_to_file("/home/sebastien/terrain.dat", &terrain);
}
