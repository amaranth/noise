// Copyright 2014 Travis Watkins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::cast;
use super::Noise;

pub struct ConstantNoise {
    value: f32,
}

impl ConstantNoise {
    pub fn new(value: f32) -> ConstantNoise {
        ConstantNoise {
            value: value,
        }
    }
}

impl Noise for ConstantNoise {
    #[allow(unused_variable)]
    fn sample_1d<F: Float>(&self, x: F) -> F {
        cast(self.value).unwrap()
    }

    #[allow(unused_variable)]
    fn sample_2d<F: Float>(&self, x: F, y: F) -> F {
        cast(self.value).unwrap()
    }

    #[allow(unused_variable)]
    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F {
        cast(self.value).unwrap()
    }
}
