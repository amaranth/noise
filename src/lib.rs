pub use self::constant::ConstantNoise;
pub use self::gradient::GradientNoise;
pub use self::integer::IntegerNoise;
pub use self::value::ValueNoise;

mod constant;
mod gradient;
mod integer;
mod value;
pub mod util;

pub trait Noise {
    fn sample_1d<F: Float>(&self, x: F) -> F;
    fn sample_2d<F: Float>(&self, x: F, y: F) -> F;
    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F;
}

#[deriving(Clone)]
pub enum Quality {
    Fast,
    Standard,
    Best,
}

