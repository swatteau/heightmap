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
extern crate clap;

use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use clap::{App, Arg};

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
    let matches = App::new("heightmap")
        .version("0.1")
        .about("Height map generator based on the diamond-square algorithm")
        .author("Sébastien Watteau")
        .arg(Arg::with_name("OUTPUT")
                .help("Path to the output file")
                .required(true))
        .arg(Arg::with_name("r")
                .long("range")
                .short("r")
                .takes_value(true)
                .number_of_values(2)
                .required(false)
                .help("Sets the range of values of the final map (defaults to [0, 255])"))
        .arg(Arg::with_name("h")
                .long("corners")
                .short("c")
                .takes_value(true)
                .number_of_values(4)
                .required(false)
                .help("Sets the initial heights at the corners of the map (defaults to 0 at each corner)"))
        .get_matches();

    let output_path = matches.value_of("OUTPUT").unwrap();
    let (range_min, range_max) = match matches.values_of("r") {
        None => (0.0, 255.0),
        Some(mut values) => {
            let min = f64::from_str(values.next().unwrap()).unwrap();
            let max = f64::from_str(values.next().unwrap()).unwrap();
            (min, max)
        }
    };
    let (h1, h2, h3, h4) = match matches.values_of("h") {
        None => (0.0, 0.0, 0.0, 0.0),
        Some(mut values) => {
            let h1 = f64::from_str(values.next().unwrap()).unwrap();
            let h2 = f64::from_str(values.next().unwrap()).unwrap();
            let h3 = f64::from_str(values.next().unwrap()).unwrap();
            let h4 = f64::from_str(values.next().unwrap()).unwrap();
            (h1, h2, h3, h4)
        }
    };

    let mut generator = TerrainGenerator::new(10, Box::new(extrinsic::PositionIndependent));
    generator.set_corners(h1, h2, h3, h4);
    let mut terrain = generator.generate();
    terrain.rescale(range_min, range_max);
    write_to_file(output_path, &terrain);
}
