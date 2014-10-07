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
    fn sample_1d<F: Float>(&self, x: F) -> F {
        cast(self.value).unwrap()
    }

    fn sample_2d<F: Float>(&self, x: F, y: F) -> F {
        cast(self.value).unwrap()
    }

    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F {
        cast(self.value).unwrap()
    }
}
