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