#![feature(iterator_step_by)]

extern crate rand;

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

pub trait ExtrinsicFn {
    fn evaluate(&mut self, p: Position, unit: usize) -> f64;
}

mod terrain;
pub mod extrinsic;

pub use terrain::{Terrain, TerrainGenerator};
