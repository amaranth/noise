use std::num::{cast, one};
use super::Noise;

static X_NOISE_GEN : int = 1619;
static Y_NOISE_GEN : int = 31337;
static Z_NOISE_GEN : int = 6971;
static SEED_NOISE_GEN : int = 1013;
static SHIFT_NOISE_GEN : uint = 8;

#[deriving(Clone)]
pub struct IntegerNoise {
    seed: int,
}

impl IntegerNoise {
    pub fn new(seed: int) -> IntegerNoise {
        IntegerNoise {
            seed: seed,
        }
    }
}

impl Noise for IntegerNoise {
    fn sample_1d<F: Float>(&self, x: F) -> F {
        let ix = x.floor().to_int().unwrap();

        let mut value : int = (
            X_NOISE_GEN * ix +
            SEED_NOISE_GEN * self.seed
            ) & 0x7fffffff;
        value ^= value >> SHIFT_NOISE_GEN;
        value = (value * (value * value * 60493 + 19990303) + 1376312589) & 0x7fffffff;

        let f_value : F = cast(value).unwrap();
        let scale = cast(1073741824.0_f32).unwrap();

        // Scale to [-1.0,1.0]
        return one::<F>() - (f_value / scale);
    }

    fn sample_2d<F: Float>(&self, x: F, y: F) -> F {
        let ix = x.floor().to_int().unwrap();
        let iy = y.floor().to_int().unwrap();

        let mut value : int = (
            X_NOISE_GEN * ix +
            Y_NOISE_GEN * iy +
            SEED_NOISE_GEN * self.seed
            ) & 0x7fffffff;
        value ^= value >> SHIFT_NOISE_GEN;
        value = (value * (value * value * 60493 + 19990303) + 1376312589) & 0x7fffffff;

        let f_value : F = cast(value).unwrap();
        let scale = cast(1073741824.0_f32).unwrap();

        // Scale to [-1.0,1.0]
        return one::<F>() - (f_value / scale);
    }

    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F {
        let ix = x.floor().to_int().unwrap();
        let iy = y.floor().to_int().unwrap();
        let iz = z.floor().to_int().unwrap();

        let mut value : int = (
            X_NOISE_GEN * ix +
            Y_NOISE_GEN * iy +
            Z_NOISE_GEN * iz +
            SEED_NOISE_GEN * self.seed
            ) & 0x7fffffff;
        value ^= value >> SHIFT_NOISE_GEN;
        value = (value * (value * value * 60493 + 19990303) + 1376312589) & 0x7fffffff;

        let f_value : F = cast(value).unwrap();
        let scale = cast(1073741824.0_f32).unwrap();

        // Scale to [-1.0,1.0]
        return one::<F>() - (f_value / scale);
    }
}
