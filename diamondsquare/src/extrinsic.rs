use super::{ExtrinsicFn, Position};

pub struct Null;

impl ExtrinsicFn for Null {
    fn evaluate(&mut self, _: Position, _: usize) -> f64 {
        0f64
    }
}