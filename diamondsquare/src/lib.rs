#![feature(iterator_step_by)]

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

pub trait ExtrinsicFn {
    fn evaluate(&mut self, p: Position, unit: usize) -> f64;
}

mod terrain;

pub use terrain::{Terrain, TerrainGenerator};
