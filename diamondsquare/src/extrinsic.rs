use super::{ExtrinsicFn, Position};

use rand;

pub struct Null;

impl ExtrinsicFn for Null {
    fn evaluate(&mut self, _: Position, _: usize) -> f64 {
        0f64
    }
}

pub struct PositionIndependent;

impl ExtrinsicFn for PositionIndependent {
    fn evaluate(&mut self, _: Position, unit: usize) -> f64 {
        let k = unit as f64;
        (0.1 * k) * (rand::random::<f64>() - 0.5)
    }
}